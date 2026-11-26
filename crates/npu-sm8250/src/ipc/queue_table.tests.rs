use super::*;

#[test]
fn encodes_the_firmware_table_in_little_endian() {
    let table = QueueTable::encode();
    let word_at = |offset| u32::from_le_bytes(table[offset..offset + 4].try_into().unwrap());
    assert_eq!(table.len(), 0x168);
    assert_eq!(word_at(0), 0x8765_4321);
    assert_eq!(word_at(20), 6);
    for (queue, data_offset) in Queue::ALL
        .into_iter()
        .zip([0x168_u32, 0x568, 0x1568, 0x2568, 0x3568, 0x4568])
    {
        let layout = QueueTable::layout(queue);
        assert_eq!(word_at(layout.descriptor_offset_bytes + 4), data_offset);
        assert_eq!(
            word_at(layout.descriptor_offset_bytes + 8),
            0x0101_0000 | queue as u32
        );
        assert_eq!(word_at(layout.descriptor_offset_bytes + 48), 0);
        assert_eq!(word_at(layout.descriptor_offset_bytes + 52), 0);
    }
}
