#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GraphRelocationError {
    InvalidRecordLength,
    UnalignedOffset,
    OffsetOutsideGraph,
    TargetOutsideWorkspace,
    AddressOverflow,
}
