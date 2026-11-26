use super::{Queue, QueueLayout};

pub struct QueueTable;

impl QueueTable {
    pub const TCM_OFFSET_BYTES: usize = 0x18000;
    pub const HEADER_BYTES: usize = 24;
    pub const DESCRIPTOR_BYTES: usize = 56;
    pub const TABLE_BYTES: usize = Self::HEADER_BYTES + Queue::ALL.len() * Self::DESCRIPTOR_BYTES;

    pub const fn layout(queue: Queue) -> QueueLayout {
        let queue_index = queue as usize;
        let mut data_offset_bytes = Self::TABLE_BYTES;
        let mut index = 0;
        while index < queue_index {
            data_offset_bytes += Queue::ALL[index].capacity_bytes();
            index += 1;
        }

        QueueLayout {
            descriptor_offset_bytes: Self::HEADER_BYTES + queue_index * Self::DESCRIPTOR_BYTES,
            data_offset_bytes,
            capacity_bytes: queue.capacity_bytes(),
        }
    }

    pub fn encode() -> [u8; Self::TABLE_BYTES] {
        const VERSION: u32 = 0x8765_4321;
        const ACTIVE: u32 = 1;
        const BIDIRECTIONAL: u32 = 0x0101_0000;

        let mut table = [0; Self::TABLE_BYTES];
        let mut output = table.chunks_exact_mut(4);
        let queue_count = Queue::ALL.len() as u32;
        let header = [
            VERSION,
            Self::TABLE_BYTES as u32,
            Self::HEADER_BYTES as u32,
            Self::DESCRIPTOR_BYTES as u32,
            queue_count,
            queue_count,
        ];
        for (word, bytes) in header.into_iter().zip(&mut output) {
            bytes.copy_from_slice(&word.to_le_bytes());
        }

        for queue in Queue::ALL {
            let layout = Self::layout(queue);
            let descriptor = [
                ACTIVE,
                layout.data_offset_bytes as u32,
                BIDIRECTIONAL | u32::from(queue as u8),
                layout.capacity_bytes as u32,
                0,
                0,
                1,
                1,
                1,
                0,
                0,
                0,
                0,
                0,
            ];
            for (word, bytes) in descriptor.into_iter().zip(&mut output) {
                bytes.copy_from_slice(&word.to_le_bytes());
            }
        }

        table
    }
}

#[cfg(test)]
#[path = "queue_table.tests.rs"]
mod tests;
