use super::{Queue, QueueError, QueueTransfer};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QueueCursor {
    capacity_bytes: usize,
    read_bytes: usize,
    write_bytes: usize,
}

impl QueueCursor {
    const WORD_BYTES: usize = 4;

    pub fn new(queue: Queue, read_bytes: usize, write_bytes: usize) -> Result<Self, QueueError> {
        let capacity_bytes = queue.capacity_bytes();
        for index in [read_bytes, write_bytes] {
            if index >= capacity_bytes || !index.is_multiple_of(Self::WORD_BYTES) {
                return Err(QueueError::InvalidIndex);
            }
        }

        Ok(Self {
            capacity_bytes,
            read_bytes,
            write_bytes,
        })
    }

    pub fn readable_bytes(self) -> usize {
        (self.capacity_bytes + self.write_bytes - self.read_bytes) % self.capacity_bytes
    }

    pub fn writable_bytes(self) -> usize {
        // An unused word keeps equal indices unambiguous.
        self.capacity_bytes - self.readable_bytes() - Self::WORD_BYTES
    }

    pub fn read(self, length_bytes: usize) -> Result<QueueTransfer, QueueError> {
        self.transfer(self.read_bytes, length_bytes, self.readable_bytes())
    }

    pub fn write(self, length_bytes: usize) -> Result<QueueTransfer, QueueError> {
        self.transfer(self.write_bytes, length_bytes, self.writable_bytes())
    }

    fn transfer(
        self,
        start_bytes: usize,
        length_bytes: usize,
        available_bytes: usize,
    ) -> Result<QueueTransfer, QueueError> {
        if length_bytes == 0
            || !length_bytes.is_multiple_of(Self::WORD_BYTES)
            || length_bytes >= self.capacity_bytes
        {
            return Err(QueueError::InvalidLength);
        }
        if length_bytes > available_bytes {
            return Err(QueueError::WouldBlock);
        }

        let first_end = (start_bytes + length_bytes).min(self.capacity_bytes);
        let second_end = length_bytes - (first_end - start_bytes);
        Ok(QueueTransfer {
            first: start_bytes..first_end,
            second: 0..second_end,
            next_index_bytes: (start_bytes + length_bytes) % self.capacity_bytes,
        })
    }
}

#[cfg(test)]
#[path = "queue_cursor.tests.rs"]
mod tests;
