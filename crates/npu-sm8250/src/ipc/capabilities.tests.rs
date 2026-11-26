use super::*;

#[test]
fn validates_reply_transaction_and_available_words() {
    let words = [36_u32, 0x10009, 0, 2, 0, 0x1005, 1, 0, 0xff];
    let mut data = [0; 36];
    for (word, bytes) in words.into_iter().zip(data.chunks_exact_mut(4)) {
        bytes.copy_from_slice(&word.to_le_bytes());
    }
    assert_eq!(Capabilities::parse(&data, 2).unwrap().words(), &[0xff]);
    assert!(Capabilities::parse(&data, 3).is_none());
    data[24] = 2;
    assert!(Capabilities::parse(&data, 2).is_none());
}
