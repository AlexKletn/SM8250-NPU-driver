#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MailboxError {
    AddressOutOfRange,
    Io,
    FirmwareRejectedPower,
    ReadbackMismatch,
}
