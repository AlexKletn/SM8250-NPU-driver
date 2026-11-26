// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2018, The Linux Foundation. All rights reserved.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ResetId {
    CalHm0Bcr = 0,
    CalHm1Bcr = 1,
    CoreBcr = 2,
    DpmTempClkAres = 3,
    DspBcr = 4,
    LlmCurrClkAres = 5,
    LlmTempClkAres = 6,
}
