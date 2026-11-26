#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExecutionBuffer {
    pub iova: u32,
    pub patch_id: u32,
}
