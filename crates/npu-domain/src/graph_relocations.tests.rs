use super::*;

#[test]
fn writes_little_endian_workspace_address() {
    let mut graph = [0; 8];
    let record = [4, 0, 0, 0, 12, 0, 0, 0];
    GraphRelocations::apply(&mut graph, &record, 0x1234_5600, 16).unwrap();
    assert_eq!(graph, [0, 0, 0, 0, 0x0c, 0x56, 0x34, 0x12]);
}

#[test]
fn late_invalid_record_leaves_graph_unchanged() {
    let mut graph = [0x99; 8];
    let records = [0, 0, 0, 0, 1, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0];
    assert_eq!(
        GraphRelocations::apply(&mut graph, &records, 0x1000, 16),
        Err(GraphRelocationError::OffsetOutsideGraph)
    );
    assert_eq!(graph, [0x99; 8]);
}

#[test]
fn rejects_wrapping_device_addresses() {
    let mut graph = [0; 4];
    let record = [0, 0, 0, 0, 4, 0, 0, 0];
    assert_eq!(
        GraphRelocations::apply(&mut graph, &record, u32::MAX - 3, 8),
        Err(GraphRelocationError::AddressOverflow)
    );
}
