use crate::GraphRelocationError;

pub struct GraphRelocations;

impl GraphRelocations {
    pub const RECORD_BYTES: usize = 8;
    const ADDRESS_BYTES: usize = 4;

    pub fn apply(
        graph: &mut [u8],
        records: &[u8],
        workspace_iova: u32,
        workspace_size: usize,
    ) -> Result<(), GraphRelocationError> {
        if !records.len().is_multiple_of(Self::RECORD_BYTES) {
            return Err(GraphRelocationError::InvalidRecordLength);
        }

        let graph_size = graph.len();
        let patches = records.chunks_exact(Self::RECORD_BYTES).map(|record| {
            let offset = u32::from_le_bytes([record[0], record[1], record[2], record[3]]) as usize;
            let target = u32::from_le_bytes([record[4], record[5], record[6], record[7]]);
            if !offset.is_multiple_of(Self::ADDRESS_BYTES) {
                return Err(GraphRelocationError::UnalignedOffset);
            }
            let end = offset
                .checked_add(Self::ADDRESS_BYTES)
                .filter(|end| *end <= graph_size)
                .ok_or(GraphRelocationError::OffsetOutsideGraph)?;
            if target as usize >= workspace_size {
                return Err(GraphRelocationError::TargetOutsideWorkspace);
            }
            let address = workspace_iova
                .checked_add(target)
                .ok_or(GraphRelocationError::AddressOverflow)?;
            Ok((offset..end, address))
        });

        // A malformed record must leave the original graph intact.
        for patch in patches.clone() {
            patch?;
        }
        for patch in patches {
            let (range, address) = patch?;
            graph[range].copy_from_slice(&address.to_le_bytes());
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "graph_relocations.tests.rs"]
mod tests;
