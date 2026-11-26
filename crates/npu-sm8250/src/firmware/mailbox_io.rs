use super::MailboxError;

pub trait MailboxIo {
    fn read_word(&mut self, offset_bytes: usize) -> Result<u32, MailboxError>;
    fn write_word(&mut self, offset_bytes: usize, word: u32) -> Result<(), MailboxError>;
}
