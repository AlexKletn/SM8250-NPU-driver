use core::ops::Range;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueueTransfer {
    pub first: Range<usize>,
    pub second: Range<usize>,
    pub next_index_bytes: usize,
}
