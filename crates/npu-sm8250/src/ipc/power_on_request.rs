pub struct PowerOnRequest;

impl PowerOnRequest {
    pub fn encode(transaction: u32) -> [u8; 24] {
        let words = [24_u32, 0x0b, transaction, 0, 0, 2];
        let mut packet = [0; 24];
        for (word, bytes) in words.into_iter().zip(packet.chunks_exact_mut(4)) {
            bytes.copy_from_slice(&word.to_le_bytes());
        }
        packet
    }
}
