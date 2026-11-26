use super::QueueError;

/// Accesses the IPC table and data through ordered 32-bit MMIO operations.
/// Offsets are relative to the start of the table; words are in host byte order.
pub trait QueueIo {
    fn read_word(&mut self, offset_bytes: usize) -> Result<u32, QueueError>;
    fn write_word(&mut self, offset_bytes: usize, word: u32) -> Result<(), QueueError>;
    fn ring_doorbell(&mut self) -> Result<(), QueueError>;
}
