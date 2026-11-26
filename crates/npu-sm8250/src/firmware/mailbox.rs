use super::{MailboxError, MailboxIo};
use crate::ipc::QueueTable;

pub struct Mailbox;

impl Mailbox {
    const FIRMWARE_STATUS: usize = 0x100;
    const HOST_STATUS: usize = 0x104;
    const HOST_VALUE: usize = 0x108;
    const FIRMWARE_EVENT: usize = 0x10c;
    const IPC_READY: u32 = 1;
    const FIRMWARE_READY: u32 = 1 << 3;

    const POWER_ACK: u32 = 1 << 16;
    const POWER_ERROR: u32 = 1 << 17;

    pub const REGISTER_BYTES: usize = Self::FIRMWARE_EVENT + 4;

    pub fn table_address(tcm_base: u64) -> Result<u32, MailboxError> {
        let address = tcm_base
            .checked_add(QueueTable::TCM_OFFSET_BYTES as u64)
            .ok_or(MailboxError::AddressOutOfRange)?;
        u32::try_from(address).map_err(|_| MailboxError::AddressOutOfRange)
    }

    pub fn reset_before_boot(io: &mut impl MailboxIo) -> Result<(), MailboxError> {
        io.write_word(Self::FIRMWARE_STATUS, 0)?;
        io.write_word(Self::HOST_VALUE, 0)?;
        io.write_word(Self::FIRMWARE_EVENT, 0)?;
        io.write_word(Self::HOST_STATUS, 0)?;
        io.read_word(Self::HOST_STATUS)?;
        Ok(())
    }

    pub fn publish_table(io: &mut impl MailboxIo, table_address: u32) -> Result<(), MailboxError> {
        io.write_word(Self::HOST_VALUE, table_address)?;
        io.write_word(Self::HOST_STATUS, Self::IPC_READY)?;
        let status = io.read_word(Self::HOST_STATUS)?;
        let address = io.read_word(Self::HOST_VALUE)?;
        if status != Self::IPC_READY || address != table_address {
            return Err(MailboxError::ReadbackMismatch);
        }
        Ok(())
    }

    pub fn clear_publication(io: &mut impl MailboxIo) -> Result<(), MailboxError> {
        io.write_word(Self::HOST_STATUS, 0)?;
        io.write_word(Self::HOST_VALUE, 0)?;
        io.read_word(Self::HOST_VALUE)?;
        Ok(())
    }

    pub fn clear_power_ack(io: &mut impl MailboxIo) -> Result<(), MailboxError> {
        let status = io.read_word(Self::FIRMWARE_STATUS)?;
        io.write_word(
            Self::FIRMWARE_STATUS,
            status & !(Self::POWER_ACK | Self::POWER_ERROR),
        )?;
        io.read_word(Self::FIRMWARE_STATUS)?;
        Ok(())
    }

    pub fn power_acknowledged(io: &mut impl MailboxIo) -> Result<bool, MailboxError> {
        let status = io.read_word(Self::FIRMWARE_STATUS)?;
        if status & Self::POWER_ERROR != 0 {
            return Err(MailboxError::FirmwareRejectedPower);
        }
        Ok(status & Self::POWER_ACK != 0)
    }

    pub fn firmware_ready(io: &mut impl MailboxIo) -> Result<bool, MailboxError> {
        Ok(io.read_word(Self::FIRMWARE_STATUS)? & Self::FIRMWARE_READY != 0)
    }
}

#[cfg(test)]
#[path = "mailbox.tests.rs"]
mod tests;
