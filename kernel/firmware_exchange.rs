// SPDX-License-Identifier: GPL-2.0-only

use kernel::{
    device::Device,
    error::{
        Result,
        code::{EIO, EPROTO, ETIMEDOUT},
    },
    prelude::*,
    time::{Delta, Instant, Monotonic, delay::fsleep},
};

use crate::{
    apps_exchange,
    core_mmio::CoreMmio,
    firmware::Mailbox,
    ipc::{Capabilities, PowerOnRequest, Queue},
    ipc_mmio::IpcMmio,
};

pub(crate) fn check_firmware_ipc(
    dev: &Device,
    mailbox: &mut CoreMmio<'_>,
    ipc: &mut IpcMmio<'_>,
) -> Result {
    Mailbox::clear_power_ack(mailbox).map_err(|_| EIO)?;
    ipc.send(Queue::HighPriority, &PowerOnRequest::encode(1))
        .map_err(|_| EIO)?;
    let start = Instant::<Monotonic>::now();
    loop {
        if Mailbox::power_acknowledged(mailbox).map_err(|_| EIO)? {
            break;
        }
        if start.elapsed() >= Delta::from_millis(5000) {
            return Err(ETIMEDOUT);
        }
        fsleep(Delta::from_millis(1));
    }

    let transaction = 2;
    let mut response = [0; Capabilities::MAX_RESPONSE_BYTES];
    let reply = apps_exchange::request(
        ipc,
        &Capabilities::request(transaction),
        transaction,
        Capabilities::REPLY_OPCODE,
        Delta::from_millis(1000),
        &mut response,
    )?;
    let caps = Capabilities::parse(reply, transaction).ok_or(EPROTO)?;
    dev_info!(dev, "NPU firmware capabilities: {:x?}\n", caps.words());
    Ok(())
}
