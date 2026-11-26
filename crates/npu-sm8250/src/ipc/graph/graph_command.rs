#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GraphCommand {
    Load,
    Execute,
    Unload,
}

impl GraphCommand {
    pub(super) const fn opcode(self) -> u32 {
        match self {
            Self::Load => 0x09,
            Self::Execute => 0x0a,
            Self::Unload => 0x02,
        }
    }

    pub const fn reply_opcode(self) -> u32 {
        match self {
            Self::Load => 0x10001,
            Self::Execute => 0x10006,
            Self::Unload => 0x10002,
        }
    }

    pub(super) fn encode(
        self,
        transaction: u32,
        payload_words: usize,
        payload: impl Iterator<Item = u32>,
        output: &mut [u8],
    ) -> Option<&mut [u8]> {
        const HEADER_WORDS: usize = 4;
        const MAX_BYTES: usize = 4092;
        let length = HEADER_WORDS.checked_add(payload_words)?.checked_mul(4)?;
        if length > MAX_BYTES {
            return None;
        }
        let packet = output.get_mut(..length)?;
        let header = [length as u32, self.opcode(), transaction, 0];
        for (word, bytes) in header
            .into_iter()
            .chain(payload)
            .zip(packet.chunks_exact_mut(4))
        {
            bytes.copy_from_slice(&word.to_le_bytes());
        }
        Some(packet)
    }
}
