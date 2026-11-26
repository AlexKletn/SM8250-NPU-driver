pub struct PowerOffRequest;

impl PowerOffRequest {
    pub const PACKET_BYTES: usize = 24;

    pub fn encode(transaction: u32) -> [u8; Self::PACKET_BYTES] {
        const NOTIFY_POWER: u32 = 0x0b;
        const PRE_CHANGE: u32 = 1;
        let words = [
            Self::PACKET_BYTES as u32,
            NOTIFY_POWER,
            transaction,
            0,
            u32::MAX,
            PRE_CHANGE,
        ];
        let mut packet = [0; Self::PACKET_BYTES];
        for (word, bytes) in words.into_iter().zip(packet.chunks_exact_mut(4)) {
            bytes.copy_from_slice(&word.to_le_bytes());
        }
        packet
    }
}
