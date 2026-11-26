// SPDX-License-Identifier: GPL-2.0-only

use kernel::io::{Io, Mmio, Region};

use crate::firmware::{MailboxError, MailboxIo};

pub(crate) struct CoreMmio<'a>(pub(crate) Mmio<'a, Region>);

impl MailboxIo for CoreMmio<'_> {
    fn read_word(&mut self, offset_bytes: usize) -> Result<u32, MailboxError> {
        self.0
            .try_read32(offset_bytes)
            .map_err(|_| MailboxError::Io)
    }

    fn write_word(&mut self, offset_bytes: usize, word: u32) -> Result<(), MailboxError> {
        self.0
            .try_write32(word, offset_bytes)
            .map_err(|_| MailboxError::Io)
    }
}
