// SPDX-License-Identifier: GPL-2.0-only

use crate::{PowerDomainId, ResetId, ResetLine};

#[unsafe(no_mangle)]
extern "C" fn sm8250_npucc_reset_offset(index: u32) -> u32 {
    let id = match index {
        0 => ResetId::CalHm0Bcr,
        1 => ResetId::CalHm1Bcr,
        2 => ResetId::CoreBcr,
        3 => ResetId::DpmTempClkAres,
        4 => ResetId::DspBcr,
        5 => ResetId::LlmCurrClkAres,
        6 => ResetId::LlmTempClkAres,
        _ => return 0,
    };
    ResetLine::for_id(id).register_offset_bytes
}

#[unsafe(no_mangle)]
extern "C" fn sm8250_npucc_reset_bit(index: u32) -> u8 {
    match index {
        3 => ResetLine::for_id(ResetId::DpmTempClkAres),
        5 => ResetLine::for_id(ResetId::LlmCurrClkAres),
        6 => ResetLine::for_id(ResetId::LlmTempClkAres),
        _ => return 0,
    }
    .mask()
    .trailing_zeros() as u8
}

#[unsafe(no_mangle)]
extern "C" fn sm8250_npucc_core_gdsc_offset() -> u32 {
    PowerDomainId::CoreGdsc.gdsc_offset_bytes()
}
