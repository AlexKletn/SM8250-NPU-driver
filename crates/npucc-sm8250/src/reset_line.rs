// SPDX-License-Identifier: GPL-2.0-only

use crate::ResetId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResetLine {
    pub register_offset_bytes: u32,
    bit_index: u8,
}

impl ResetLine {
    pub const fn for_id(id: ResetId) -> Self {
        let (register_offset_bytes, bit_index) = match id {
            ResetId::CalHm0Bcr => (0x10f0, 0),
            ResetId::CalHm1Bcr => (0x1130, 0),
            ResetId::CoreBcr => (0x1000, 0),
            ResetId::DpmTempClkAres => (0x10c4, 2),
            ResetId::DspBcr => (0x1200, 0),
            ResetId::LlmCurrClkAres => (0x10d4, 2),
            ResetId::LlmTempClkAres => (0x10c8, 2),
        };

        Self {
            register_offset_bytes,
            bit_index,
        }
    }

    pub const fn mask(self) -> u32 {
        1 << self.bit_index
    }
}
