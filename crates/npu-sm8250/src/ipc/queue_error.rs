#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueueError {
    Io,
    InvalidIndex,
    InvalidLength,
    BufferTooSmall,
    WouldBlock,
}
