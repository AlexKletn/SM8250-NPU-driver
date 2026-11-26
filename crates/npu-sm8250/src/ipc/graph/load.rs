use super::{GraphCommand, LoadPatch};

pub struct Load<'a> {
    pub network_id: u32,
    pub first_block_size_bytes: u32,
    pub graph_iova: u32,
    pub layer_count: u32,
    pub patches: &'a [LoadPatch],
}

impl Load<'_> {
    pub fn encode<'out>(&self, transaction: u32, output: &'out mut [u8]) -> Option<&'out mut [u8]> {
        let count = u32::try_from(self.patches.len()).ok()?;
        let header = [
            self.network_id,
            self.first_block_size_bytes,
            self.graph_iova,
            self.layer_count,
            count,
        ];
        let payload_words = header
            .len()
            .checked_add(self.patches.len().checked_mul(6)?)?;
        let payload = header
            .into_iter()
            .chain(self.patches.iter().copied().flat_map(LoadPatch::words));
        GraphCommand::Load.encode(transaction, payload_words, payload, output)
    }
}
