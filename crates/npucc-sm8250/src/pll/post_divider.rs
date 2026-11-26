// SPDX-License-Identifier: GPL-2.0-only

use crate::ClockId;

use super::LucidPll;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PostDivider {
    pub parent: ClockId,
    pub pll: LucidPll,
}

impl PostDivider {
    pub const SHIFT: u8 = 8;
    pub const WIDTH: u8 = 4;
    pub const ENCODING: u8 = 0;
    pub const DIVISOR: u8 = 1;

    pub const fn for_id(id: ClockId) -> Option<Self> {
        let parent = match id {
            ClockId::Pll0OutEven => ClockId::Pll0,
            ClockId::Pll1OutEven => ClockId::Pll1,
            _ => return None,
        };
        let Some(pll) = LucidPll::for_id(parent) else {
            return None;
        };

        Some(Self { parent, pll })
    }
}
