// SPDX-License-Identifier: GPL-2.0-only

use kernel::{
    alloc::flags::GFP_KERNEL,
    error::{
        Result,
        code::{EIO, EMSGSIZE, EPROTO},
    },
    prelude::*,
    time::Delta,
};

use crate::{
    apps_exchange,
    ipc::{
        Queue,
        graph::{Execute, GraphCommand, GraphReply, Load, Unload},
    },
    ipc_mmio::IpcMmio,
};

pub(crate) struct GraphExchange<'io, 'mmio> {
    ipc: &'io mut IpcMmio<'mmio>,
    packet: KVec<u8>,
    response: KVec<u8>,
    transaction: u32,
}

impl<'io, 'mmio> GraphExchange<'io, 'mmio> {
    pub(crate) fn new(ipc: &'io mut IpcMmio<'mmio>) -> Result<Self> {
        let mut packet = KVec::new();
        packet.resize(Queue::AppsCommand.capacity_bytes() - 4, 0, GFP_KERNEL)?;
        let mut response = KVec::new();
        response.resize(Queue::AppsResponse.capacity_bytes() - 4, 0, GFP_KERNEL)?;
        Ok(Self {
            ipc,
            packet,
            response,
            transaction: 2,
        })
    }

    pub(crate) fn load(&mut self, load: &Load<'_>) -> Result<u32> {
        let transaction = self.next_transaction();
        let packet = load.encode(transaction, &mut self.packet).ok_or(EMSGSIZE)?;
        let handle = Self::exchange(
            self.ipc,
            packet,
            &mut self.response,
            GraphCommand::Load,
            transaction,
        )?;
        if handle == 0 {
            return Err(EPROTO);
        }
        Ok(handle)
    }

    pub(crate) fn execute(&mut self, execute: &Execute<'_>) -> Result {
        let transaction = self.next_transaction();
        let packet = execute
            .encode(transaction, &mut self.packet)
            .ok_or(EMSGSIZE)?;
        let handle = Self::exchange(
            self.ipc,
            packet,
            &mut self.response,
            GraphCommand::Execute,
            transaction,
        )?;
        Self::check_handle(execute.handle, handle)
    }

    pub(crate) fn unload(&mut self, handle: u32) -> Result {
        let transaction = self.next_transaction();
        let packet = Unload::encode(transaction, handle);
        let reply_handle = Self::exchange(
            self.ipc,
            &packet,
            &mut self.response,
            GraphCommand::Unload,
            transaction,
        )?;
        Self::check_handle(handle, reply_handle)
    }

    fn exchange(
        ipc: &mut IpcMmio<'_>,
        packet: &[u8],
        response: &mut [u8],
        command: GraphCommand,
        transaction: u32,
    ) -> Result<u32> {
        let reply = apps_exchange::request(
            ipc,
            packet,
            transaction,
            command.reply_opcode(),
            Delta::from_millis(5000),
            response,
        )?;
        let Some(reply) = GraphReply::parse(reply, command, transaction) else {
            pr_err!(
                "NPU {command:?} reply did not match transaction {transaction}: {:02x?}\n",
                &reply[..reply.len().min(GraphReply::HEADER_BYTES)]
            );
            return Err(EPROTO);
        };
        if reply.firmware_status != 0 {
            pr_err!(
                "NPU {command:?} firmware status {:#x}\n",
                reply.firmware_status
            );
            return Err(EIO);
        }
        Ok(reply.handle)
    }

    fn next_transaction(&mut self) -> u32 {
        self.transaction = self.transaction.wrapping_add(1).max(1);
        self.transaction
    }

    fn check_handle(expected: u32, actual: u32) -> Result {
        if expected != actual {
            return Err(EPROTO);
        }
        Ok(())
    }
}
