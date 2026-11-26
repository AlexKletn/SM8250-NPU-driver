use super::*;

#[derive(Default)]
struct Memory {
    words: [u32; 4],
    read: u32,
    write: u32,
    doorbells: u32,
}

impl QueueIo for Memory {
    fn read_word(&mut self, offset: usize) -> Result<u32, QueueError> {
        Ok(match offset {
            0x38 | 0x3c => 1,
            0x48 => self.read,
            0x4c => self.write,
            0x564 => self.words[0],
            0x168 => self.words[1],
            0x16c => self.words[2],
            _ => panic!("unexpected read {offset:#x}"),
        })
    }

    fn write_word(&mut self, offset: usize, word: u32) -> Result<(), QueueError> {
        match offset {
            0x38 => assert_eq!(word, 1),
            0x3c => assert_eq!(word, 0),
            0x48 => self.read = word,
            0x4c => {
                assert_eq!(self.words[..3], [12, 0x0403_0201, 0x0807_0605]);
                self.write = word;
            }
            0x564 => self.words[0] = word,
            0x168 => self.words[1] = word,
            0x16c => self.words[2] = word,
            _ => panic!("unexpected write {offset:#x}"),
        }
        Ok(())
    }

    fn ring_doorbell(&mut self) -> Result<(), QueueError> {
        assert_eq!(self.write, 8);
        self.doorbells += 1;
        Ok(())
    }
}

#[test]
fn publishes_wrapped_packet_before_notifying_firmware() {
    let mut memory = Memory {
        read: 1020,
        write: 1020,
        ..Memory::default()
    };
    let packet = [12, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8];
    QueueTransport::send(&mut memory, Queue::HighPriority, &packet).unwrap();
    assert_eq!(memory.doorbells, 1);
    let mut output = [0; 12];
    assert_eq!(
        QueueTransport::receive(&mut memory, Queue::HighPriority, &mut output, 12),
        Ok(12)
    );
    assert_eq!(output, packet);
    assert_eq!(memory.read, memory.write);
    assert_eq!(memory.doorbells, 2);
}

#[test]
fn undersized_output_does_not_consume_packet() {
    let mut memory = Memory {
        words: [12, 0, 0, 0],
        read: 1020,
        write: 8,
        ..Memory::default()
    };
    assert_eq!(
        QueueTransport::receive(&mut memory, Queue::HighPriority, &mut [0; 4], 12),
        Err(QueueError::BufferTooSmall)
    );
    assert_eq!(memory.read, 1020);
    assert_eq!(memory.doorbells, 0);
}
