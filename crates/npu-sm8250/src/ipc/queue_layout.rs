#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QueueLayout {
    pub descriptor_offset_bytes: usize,
    pub data_offset_bytes: usize,
    pub capacity_bytes: usize,
}
