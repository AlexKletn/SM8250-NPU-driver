// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2018, The Linux Foundation. All rights reserved.

use crate::{ClockId, ClockParent};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LucidPll {
    pub register_offset_bytes: u32,
    pub l: u32,
    pub alpha: u32,
}

impl LucidPll {
    pub const PARENT: ClockParent = ClockParent::BiTcxo;
    pub const VCO_MIN_HZ: u32 = 249_600_000;
    pub const VCO_MAX_HZ: u32 = 2_000_000_000;
    pub const VCO_SELECTOR: u8 = 0;
    pub const CALIBRATION_L: u32 = 0x44;
    pub const CONFIG_CTL: u32 = 0x2048_5699;
    pub const CONFIG_CTL_HI: u32 = 0x0000_2261;
    pub const CONFIG_CTL_HI1: u32 = 0x329a_699c;
    pub const USER_CTL: u32 = 0;
    pub const USER_CTL_HI: u32 = 0x0000_0805;
    pub const USER_CTL_HI1: u32 = 0;

    pub const fn for_id(id: ClockId) -> Option<Self> {
        let (register_offset_bytes, l, alpha) = match id {
            ClockId::Pll0 => (0x00000, 0x1f, 0x4000),
            ClockId::Pll1 => (0x00400, 0x4e, 0x2000),
            ClockId::Q6SsPll => (0x20000, 0x0f, 0xa000),
            _ => return None,
        };
        Some(Self {
            register_offset_bytes,
            l,
            alpha,
        })
    }
}

#[cfg(test)]
#[path = "lucid_pll.tests.rs"]
mod tests;
