use super::*;

fn packet(opcode: u32, transaction: u32, bytes: usize) -> [u8; 36] {
    let words = [bytes as u32, opcode, 0, transaction];
    let mut packet = [0; 36];
    for (word, output) in words.into_iter().zip(packet.chunks_exact_mut(4)) {
        output.copy_from_slice(&word.to_le_bytes());
    }
    packet
}

#[test]
fn asynchronous_notification_cannot_complete_execute() {
    let older_notification = packet(0x10010, 2, 36);
    let matching_id_notification = packet(0x10010, 3, 36);
    let execute_done = packet(0x10006, 3, 24);
    let responses = [
        older_notification.as_slice(),
        matching_id_notification.as_slice(),
        &execute_done[..24],
    ];

    let matching = responses
        .iter()
        .position(|bytes| ReplyHeader::parse(bytes).unwrap().matches(0x10006, 3));

    assert_eq!(matching, Some(2));
}

#[test]
fn rejects_truncated_packets() {
    let response = packet(0x10006, 3, 24);

    assert_eq!(ReplyHeader::parse(&response[..20]), None);
}
