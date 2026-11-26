#[derive(Debug, Eq, PartialEq)]
pub struct ReplyHeader {
    opcode: u32,
    transaction: u32,
}

impl ReplyHeader {
    pub const MIN_PACKET_BYTES: usize = 20;

    pub fn parse(packet: &[u8]) -> Option<Self> {
        if packet.len() < Self::MIN_PACKET_BYTES || !packet.len().is_multiple_of(4) {
            return None;
        }
        let mut words = packet
            .chunks_exact(4)
            .map(|word| u32::from_le_bytes([word[0], word[1], word[2], word[3]]));
        let size = words.next()?;
        if size as usize != packet.len() {
            return None;
        }
        let opcode = words.next()?;
        let transaction = words.nth(1)?;
        Some(Self {
            opcode,
            transaction,
        })
    }

    pub fn matches(&self, opcode: u32, transaction: u32) -> bool {
        self.opcode == opcode && self.transaction == transaction
    }
}

#[cfg(test)]
#[path = "reply_header.tests.rs"]
mod tests;
