use super::*;

#[derive(Default)]
struct Registers {
    host_value: u32,
    host_status: u32,
}

impl MailboxIo for Registers {
    fn read_word(&mut self, offset: usize) -> Result<u32, MailboxError> {
        Ok(match offset {
            0x104 => self.host_status,
            0x108 => self.host_value,
            _ => panic!("unexpected read"),
        })
    }

    fn write_word(&mut self, offset: usize, word: u32) -> Result<(), MailboxError> {
        match offset {
            0x108 => self.host_value = word,
            0x104 => {
                if word != 0 {
                    assert_ne!(self.host_value, 0);
                }
                self.host_status = word;
            }
            _ => panic!("unexpected write"),
        }
        Ok(())
    }
}

#[test]
fn publishes_address_before_ready_and_clears_both() {
    let mut registers = Registers::default();
    let address = Mailbox::table_address(0x0990_0000).unwrap();
    assert_eq!(address, 0x0991_8000);
    Mailbox::publish_table(&mut registers, address).unwrap();
    assert_eq!(registers.host_status, 1);
    assert_eq!(registers.host_value, address);
    Mailbox::clear_publication(&mut registers).unwrap();
    assert_eq!((registers.host_status, registers.host_value), (0, 0));
}

#[test]
fn rejects_addresses_that_cannot_reach_the_firmware() {
    for address in [u32::MAX as u64, u64::MAX] {
        assert_eq!(
            Mailbox::table_address(address),
            Err(MailboxError::AddressOutOfRange)
        );
    }
}
