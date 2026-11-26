// SPDX-License-Identifier: GPL-2.0-only

use crate::ClockId;

pub struct CalibrationDivider;

impl CalibrationDivider {
    pub const PARENT: ClockId = ClockId::Pll0OutEven;
    pub const MULTIPLIER: u8 = 1;
    pub const DIVISOR: u8 = 2;

    pub const REGISTER_VALUES: [(u32, u32); 4] = [
        (0x11a4, 0x0001_5011),
        (0x11a0, 0x0080_0000),
        (0x11b4, 0x0001_5011),
        (0x11b0, 0x0080_0000),
    ];
}
