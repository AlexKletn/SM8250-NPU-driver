use super::{Queue, QueueCursor, QueueError, QueueIo, QueueLayout, QueueTable, QueueTransfer};

pub struct QueueTransport;

impl QueueTransport {
    const RX_REQUEST: usize = 0x20;
    const TX_REQUEST: usize = 0x24;
    const READ_INDEX: usize = 0x30;
    const WRITE_INDEX: usize = 0x34;

    pub fn send(io: &mut impl QueueIo, queue: Queue, packet: &[u8]) -> Result<(), QueueError> {
        if packet.len() < 4
            || u32::from_le_bytes([packet[0], packet[1], packet[2], packet[3]]) as usize
                != packet.len()
        {
            return Err(QueueError::InvalidLength);
        }
        let layout = QueueTable::layout(queue);
        let cursor = Self::cursor(io, queue, layout)?;
        let transfer = cursor.write(packet.len())?;

        for (offset, bytes) in Self::word_offsets(layout, &transfer).zip(packet.chunks_exact(4)) {
            io.write_word(
                offset,
                u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            )?;
        }
        let descriptor = layout.descriptor_offset_bytes;
        io.write_word(descriptor + Self::TX_REQUEST, 0)?;
        io.write_word(
            descriptor + Self::WRITE_INDEX,
            transfer.next_index_bytes as u32,
        )?;
        if io.read_word(descriptor + Self::RX_REQUEST)? != 0 {
            io.ring_doorbell()?;
        }
        Ok(())
    }

    pub fn receive(
        io: &mut impl QueueIo,
        queue: Queue,
        output: &mut [u8],
        minimum_packet_bytes: usize,
    ) -> Result<usize, QueueError> {
        let layout = QueueTable::layout(queue);
        let descriptor = layout.descriptor_offset_bytes;
        let cursor = Self::cursor(io, queue, layout)?;
        let header = cursor.read(4)?;
        let length_bytes = io.read_word(layout.data_offset_bytes + header.first.start)? as usize;
        if length_bytes < minimum_packet_bytes.max(4) || length_bytes > cursor.readable_bytes() {
            return Err(QueueError::InvalidLength);
        }
        let transfer = cursor.read(length_bytes)?;
        let packet = output
            .get_mut(..length_bytes)
            .ok_or(QueueError::BufferTooSmall)?;

        for (offset, bytes) in Self::word_offsets(layout, &transfer).zip(packet.chunks_exact_mut(4))
        {
            bytes.copy_from_slice(&io.read_word(offset)?.to_le_bytes());
        }
        io.write_word(
            descriptor + Self::READ_INDEX,
            transfer.next_index_bytes as u32,
        )?;
        io.write_word(
            descriptor + Self::RX_REQUEST,
            u32::from(length_bytes == cursor.readable_bytes()),
        )?;
        if io.read_word(descriptor + Self::TX_REQUEST)? != 0 {
            io.ring_doorbell()?;
        }
        Ok(length_bytes)
    }

    fn cursor(
        io: &mut impl QueueIo,
        queue: Queue,
        layout: QueueLayout,
    ) -> Result<QueueCursor, QueueError> {
        let descriptor = layout.descriptor_offset_bytes;
        let read = io.read_word(descriptor + Self::READ_INDEX)? as usize;
        let write = io.read_word(descriptor + Self::WRITE_INDEX)? as usize;
        QueueCursor::new(queue, read, write)
    }

    fn word_offsets(layout: QueueLayout, transfer: &QueueTransfer) -> impl Iterator<Item = usize> {
        transfer
            .first
            .clone()
            .step_by(4)
            .chain(transfer.second.clone().step_by(4))
            .map(move |offset| layout.data_offset_bytes + offset)
    }
}

#[cfg(test)]
#[path = "queue_transport.tests.rs"]
mod tests;
