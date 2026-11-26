// SPDX-License-Identifier: GPL-2.0-only

use kernel::{
    error::{Result, code::EINVAL},
    io::{Io, Mmio, Region},
};

use crate::ipc::{Queue, QueueError, QueueIo, QueueTable, QueueTransport};

pub(crate) struct IpcMmio<'a> {
    tcm: Mmio<'a, Region>,
    apss_shared: Mmio<'a, Region>,
}

impl<'a> IpcMmio<'a> {
    const DOORBELL_OFFSET_BYTES: usize = 0x10;
    const DOORBELL_APPS: u32 = 1 << 6;

    pub(crate) fn new(tcm: Mmio<'a, Region>, apss_shared: Mmio<'a, Region>) -> Result<Self> {
        let last_queue = QueueTable::layout(Queue::FirmwareLog);
        let table_end =
            QueueTable::TCM_OFFSET_BYTES + last_queue.data_offset_bytes + last_queue.capacity_bytes;
        if tcm.size() < table_end || apss_shared.size() < Self::DOORBELL_OFFSET_BYTES + 4 {
            return Err(EINVAL);
        }

        Ok(Self { tcm, apss_shared })
    }

    pub(crate) fn initialize_table(&mut self) -> core::result::Result<(), QueueError> {
        let table = QueueTable::encode();
        for (index, bytes) in table.chunks_exact(4).enumerate() {
            self.write_word(
                index * 4,
                u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            )?;
        }
        for (index, bytes) in table.chunks_exact(4).enumerate() {
            if self.read_word(index * 4)?.to_le_bytes() != bytes {
                return Err(QueueError::Io);
            }
        }
        Ok(())
    }

    pub(crate) fn send(
        &mut self,
        queue: Queue,
        packet: &[u8],
    ) -> core::result::Result<(), QueueError> {
        QueueTransport::send(self, queue, packet)
    }

    pub(crate) fn receive(
        &mut self,
        queue: Queue,
        output: &mut [u8],
        minimum_packet_bytes: usize,
    ) -> core::result::Result<usize, QueueError> {
        QueueTransport::receive(self, queue, output, minimum_packet_bytes)
    }
}

impl QueueIo for IpcMmio<'_> {
    fn read_word(&mut self, offset_bytes: usize) -> core::result::Result<u32, QueueError> {
        self.tcm
            .try_read32(
                QueueTable::TCM_OFFSET_BYTES
                    .checked_add(offset_bytes)
                    .ok_or(QueueError::Io)?,
            )
            .map_err(|_| QueueError::Io)
    }

    fn write_word(
        &mut self,
        offset_bytes: usize,
        word: u32,
    ) -> core::result::Result<(), QueueError> {
        self.tcm
            .try_write32(
                word,
                QueueTable::TCM_OFFSET_BYTES
                    .checked_add(offset_bytes)
                    .ok_or(QueueError::Io)?,
            )
            .map_err(|_| QueueError::Io)
    }

    fn ring_doorbell(&mut self) -> core::result::Result<(), QueueError> {
        self.apss_shared
            .try_write32(Self::DOORBELL_APPS, Self::DOORBELL_OFFSET_BYTES)
            .map_err(|_| QueueError::Io)
    }
}
