// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2018, The Linux Foundation. All rights reserved.

use crate::ClockId;

use super::{ClockParent, ClockParent::*, ClockRate};

const MAIN_PARENTS: &[(ClockParent, u8)] = &[
    (BiTcxo, 0),
    (Pll1Even, 1),
    (Pll0Even, 2),
    (GccGpll0, 4),
    (GccGpll0Divider, 5),
    (CorePllTest, 7),
];

const CALIBRATION_PARENTS: &[(ClockParent, u8)] = &[
    (BiTcxo, 0),
    (Pll1Even, 1),
    (CrcDivider, 2),
    (GccGpll0, 4),
    (GccGpll0Divider, 5),
    (CorePllTest, 7),
];

const XO_PARENTS: &[(ClockParent, u8)] = &[(BiTcxoAlwaysOn, 0), (CorePllTest, 7)];

const DSP_PARENTS: &[(ClockParent, u8)] = &[(BiTcxo, 0), (Q6ssPllMain, 2), (CorePllTest, 7)];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ClockSource {
    pub command_offset_bytes: u32,
    pub hardware_clock_control: bool,
    pub set_rate_parent: bool,
    pub companion: Option<ClockId>,
    pub parents: &'static [(ClockParent, u8)],
    pub rates: &'static [ClockRate],
}

impl ClockSource {
    pub const HID_WIDTH: u8 = 5;

    pub const fn for_id(id: ClockId) -> Option<Self> {
        let (command_offset_bytes, parents) = match id {
            ClockId::CalHm0ClkSrc => (0x1100, CALIBRATION_PARENTS),
            ClockId::CalHm1ClkSrc => (0x1140, CALIBRATION_PARENTS),
            ClockId::CoreClkSrc => (0x1010, MAIN_PARENTS),
            ClockId::LmhClkSrc => (0x1060, MAIN_PARENTS),
            ClockId::XoClkSrc => (0x1400, XO_PARENTS),
            ClockId::DspCoreClkSrc => (0x10028, DSP_PARENTS),
            _ => return None,
        };

        Some(Self {
            command_offset_bytes,
            hardware_clock_control: !matches!(id, ClockId::XoClkSrc),
            set_rate_parent: !matches!(id, ClockId::XoClkSrc),
            companion: match id {
                ClockId::CalHm0ClkSrc => Some(ClockId::CalHm1ClkSrc),
                _ => None,
            },
            parents,
            rates: id.source_rates(),
        })
    }
}

#[cfg(test)]
#[path = "clock_source.tests.rs"]
mod tests;
