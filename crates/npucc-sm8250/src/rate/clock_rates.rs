// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2018, The Linux Foundation. All rights reserved.

use super::{ClockParent::*, ClockRate};
use crate::ClockId;

const CALIBRATION_RATES: &[ClockRate] = &[
    ClockRate::new(300_000_000, CrcDivider, 2),
    ClockRate::new(406_000_000, CrcDivider, 2),
    ClockRate::new(533_000_000, CrcDivider, 2),
    ClockRate::new(730_000_000, CrcDivider, 2),
    ClockRate::new(920_000_000, CrcDivider, 2),
    ClockRate::new(1_000_000_000, CrcDivider, 2),
];

const CORE_RATES: &[ClockRate] = &[
    ClockRate::new(100_000_000, GccGpll0Divider, 6),
    ClockRate::new(200_000_000, GccGpll0, 6),
    ClockRate::new(333_333_333, Pll1Even, 9),
    ClockRate::new(428_571_429, Pll1Even, 7),
    ClockRate::new(500_000_000, Pll1Even, 6),
];

const LMH_RATES: &[ClockRate] = &[
    ClockRate::new(100_000_000, GccGpll0Divider, 6),
    ClockRate::new(200_000_000, GccGpll0, 6),
    ClockRate::new(214_285_714, Pll1Even, 14),
    ClockRate::new(300_000_000, Pll1Even, 10),
];

const XO_RATES: &[ClockRate] = &[ClockRate::new(19_200_000, BiTcxoAlwaysOn, 2)];

const DSP_RATES: &[ClockRate] = &[
    ClockRate::new(300_000_000, Q6ssPllMain, 2),
    ClockRate::new(400_000_000, Q6ssPllMain, 2),
    ClockRate::new(500_000_000, Q6ssPllMain, 2),
    ClockRate::new(660_000_000, Q6ssPllMain, 2),
    ClockRate::new(800_000_000, Q6ssPllMain, 2),
];

impl ClockId {
    pub const fn source_rates(self) -> &'static [ClockRate] {
        match self {
            Self::CalHm0ClkSrc | Self::CalHm1ClkSrc => CALIBRATION_RATES,
            Self::CoreClkSrc => CORE_RATES,
            Self::LmhClkSrc => LMH_RATES,
            Self::XoClkSrc => XO_RATES,
            Self::DspCoreClkSrc => DSP_RATES,
            _ => &[],
        }
    }
}

#[cfg(test)]
#[path = "clock_rates.tests.rs"]
mod tests;
