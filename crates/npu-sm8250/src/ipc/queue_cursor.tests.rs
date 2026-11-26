use super::*;

#[test]
fn wrapped_transfer_preserves_fifo_order() {
    let tx = QueueCursor::new(Queue::AppsCommand, 16, 4092).unwrap();
    let transfer = tx.write(12).unwrap();
    assert_eq!(transfer.first, 4092..4096);
    assert_eq!(transfer.second, 0..8);
    let rx = QueueCursor::new(Queue::AppsCommand, 4092, transfer.next_index_bytes).unwrap();
    assert_eq!(rx.read(12).unwrap(), transfer);
}

#[test]
fn equal_indices_mean_empty_and_a_full_ring_retains_one_word() {
    let empty = QueueCursor::new(Queue::HighPriority, 0, 0).unwrap();
    assert_eq!(empty.read(4), Err(QueueError::WouldBlock));
    assert_eq!(empty.writable_bytes(), 1020);

    let full = QueueCursor::new(Queue::HighPriority, 0, 1020).unwrap();
    assert_eq!(full.writable_bytes(), 0);
    assert_eq!(full.write(4), Err(QueueError::WouldBlock));
}

#[test]
fn rejects_bad_indices_and_lengths() {
    assert_eq!(
        QueueCursor::new(Queue::HighPriority, 1024, 0),
        Err(QueueError::InvalidIndex)
    );
    assert_eq!(
        QueueCursor::new(Queue::HighPriority, 0, 3),
        Err(QueueError::InvalidIndex)
    );
    let cursor = QueueCursor::new(Queue::HighPriority, 0, 0).unwrap();
    for length in [0, 3, 1024, usize::MAX] {
        assert_eq!(cursor.write(length), Err(QueueError::InvalidLength));
    }
}
