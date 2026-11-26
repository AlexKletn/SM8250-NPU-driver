// SPDX-License-Identifier: GPL-2.0-only

use kernel::{
    error::{
        Result,
        code::{EAGAIN, EIO, EPROTO, ETIMEDOUT},
    },
    time::{Delta, Instant, Monotonic, delay::fsleep},
};

use crate::{
    ipc::{Queue, QueueError, ReplyHeader},
    ipc_mmio::IpcMmio,
};

pub(crate) fn request<'out>(
    ipc: &mut IpcMmio<'_>,
    packet: &[u8],
    transaction: u32,
    expected_opcode: u32,
    timeout: Delta,
    response: &'out mut [u8],
) -> Result<&'out [u8]> {
    ipc.send(Queue::AppsCommand, packet).map_err(queue_error)?;
    let start = Instant::<Monotonic>::now();
    while start.elapsed() < timeout {
        match ipc.receive(Queue::AppsResponse, response, ReplyHeader::MIN_PACKET_BYTES) {
            Ok(length) => {
                let header = ReplyHeader::parse(&response[..length]).ok_or(EPROTO)?;
                if header.matches(expected_opcode, transaction) {
                    return Ok(&response[..length]);
                }
            }
            Err(QueueError::WouldBlock) => fsleep(Delta::from_millis(1)),
            Err(error) => return Err(queue_error(error)),
        }
    }
    Err(ETIMEDOUT)
}

fn queue_error(error: QueueError) -> kernel::error::Error {
    match error {
        QueueError::Io => EIO,
        QueueError::WouldBlock => EAGAIN,
        QueueError::BufferTooSmall | QueueError::InvalidIndex | QueueError::InvalidLength => EPROTO,
    }
}
