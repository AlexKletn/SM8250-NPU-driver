use super::{ExecutionBuffer, GraphCommand};

pub struct Execute<'a> {
    pub handle: u32,
    pub buffers: &'a [ExecutionBuffer],
}

impl Execute<'_> {
    pub fn encode<'out>(&self, transaction: u32, output: &'out mut [u8]) -> Option<&'out mut [u8]> {
        let count = u32::try_from(self.buffers.len()).ok()?;
        let header = [self.handle, count];
        let payload_words = header
            .len()
            .checked_add(self.buffers.len().checked_mul(2)?)?;
        let payload = header.into_iter().chain(
            self.buffers
                .iter()
                .flat_map(|buffer| [buffer.iova, buffer.patch_id]),
        );
        GraphCommand::Execute.encode(transaction, payload_words, payload, output)
    }
}
