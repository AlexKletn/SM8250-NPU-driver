pub struct Capabilities {
    words: [u32; Self::MAX_WORDS],
    count: usize,
}

impl Capabilities {
    pub const REPLY_OPCODE: u32 = 0x10009;
    const PROPERTY: u32 = 0x1005;
    const MAX_WORDS: usize = 8;
    const RESPONSE_HEADER_WORDS: usize = 8;
    pub const MIN_RESPONSE_BYTES: usize = Self::RESPONSE_HEADER_WORDS * 4;
    pub const MAX_RESPONSE_BYTES: usize = (Self::RESPONSE_HEADER_WORDS + Self::MAX_WORDS) * 4;

    pub fn request(transaction: u32) -> [u8; 60] {
        let mut packet = [0; 60];
        let words = [
            60_u32,
            0x0d,
            transaction,
            0,
            Self::PROPERTY,
            Self::MAX_WORDS as u32,
        ];
        for (word, bytes) in words.into_iter().zip(packet.chunks_exact_mut(4)) {
            bytes.copy_from_slice(&word.to_le_bytes());
        }
        packet
    }

    pub fn parse(packet: &[u8], transaction: u32) -> Option<Self> {
        if !(Self::MIN_RESPONSE_BYTES..=Self::MAX_RESPONSE_BYTES).contains(&packet.len())
            || !packet.len().is_multiple_of(4)
        {
            return None;
        }
        let mut words = packet
            .chunks_exact(4)
            .map(|bytes| u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]));
        let [
            size,
            opcode,
            status,
            reply_transaction,
            _,
            property,
            count,
            network,
        ] = core::array::from_fn(|_| words.next().unwrap_or_default());
        let count = count as usize;
        if size as usize != packet.len()
            || opcode != Self::REPLY_OPCODE
            || status != 0
            || reply_transaction != transaction
            || property != Self::PROPERTY
            || network != 0
            || count > words.len()
        {
            return None;
        }
        let mut capabilities = Self {
            words: [0; Self::MAX_WORDS],
            count,
        };
        for (dest, word) in capabilities.words[..count].iter_mut().zip(words) {
            *dest = word;
        }
        Some(capabilities)
    }

    pub fn words(&self) -> &[u32] {
        &self.words[..self.count]
    }
}

#[cfg(test)]
#[path = "capabilities.tests.rs"]
mod tests;
