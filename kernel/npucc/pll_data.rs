// SPDX-License-Identifier: GPL-2.0-only

use crate::{CalibrationDivider, ClockId, LucidPll, PostDivider};

#[repr(C)]
pub(crate) struct PllData {
    offset: u32,
    l: u32,
    alpha: u32,
    calibration_l: u32,
    config_ctl: u32,
    config_ctl_hi: u32,
    config_ctl_hi1: u32,
    user_ctl: u32,
    user_ctl_hi: u32,
    user_ctl_hi1: u32,
    vco_min_hz: u32,
    vco_max_hz: u32,
    vco_selector: u8,
}

#[unsafe(no_mangle)]
extern "C" fn sm8250_npucc_pll_data(index: u32, output: *mut PllData) -> bool {
    let Some(pll) = ClockId::from_index(index as usize).and_then(LucidPll::for_id) else {
        return false;
    };
    if output.is_null() {
        return false;
    }
    // SAFETY: C passes aligned, writable storage for one PLL descriptor.
    unsafe {
        output.write(PllData {
            offset: pll.register_offset_bytes,
            l: pll.l,
            alpha: pll.alpha,
            calibration_l: LucidPll::CALIBRATION_L,
            config_ctl: LucidPll::CONFIG_CTL,
            config_ctl_hi: LucidPll::CONFIG_CTL_HI,
            config_ctl_hi1: LucidPll::CONFIG_CTL_HI1,
            user_ctl: LucidPll::USER_CTL,
            user_ctl_hi: LucidPll::USER_CTL_HI,
            user_ctl_hi1: LucidPll::USER_CTL_HI1,
            vco_min_hz: LucidPll::VCO_MIN_HZ,
            vco_max_hz: LucidPll::VCO_MAX_HZ,
            vco_selector: LucidPll::VCO_SELECTOR,
        });
    }
    true
}

#[repr(C)]
pub(crate) struct PostDividerData {
    offset: u32,
    parent: u32,
    shift: u8,
    width: u8,
    encoding: u8,
    divisor: u8,
}

#[unsafe(no_mangle)]
extern "C" fn sm8250_npucc_post_divider_data(index: u32, output: *mut PostDividerData) -> bool {
    let Some(divider) = ClockId::from_index(index as usize).and_then(PostDivider::for_id) else {
        return false;
    };
    if output.is_null() {
        return false;
    }
    // SAFETY: C passes aligned, writable storage for one divider descriptor.
    unsafe {
        output.write(PostDividerData {
            offset: divider.pll.register_offset_bytes,
            parent: divider.parent as u32,
            shift: PostDivider::SHIFT,
            width: PostDivider::WIDTH,
            encoding: PostDivider::ENCODING,
            divisor: PostDivider::DIVISOR,
        });
    }
    true
}

#[repr(C)]
pub(crate) struct CalibrationData {
    parent: u32,
    multiplier: u8,
    divisor: u8,
    register_offsets: [u32; 4],
    register_values: [u32; 4],
}

#[unsafe(no_mangle)]
extern "C" fn sm8250_npucc_calibration_data(output: *mut CalibrationData) {
    if output.is_null() {
        return;
    }
    let registers = CalibrationDivider::REGISTER_VALUES;
    // SAFETY: C passes aligned, writable storage for one calibration descriptor.
    unsafe {
        output.write(CalibrationData {
            parent: CalibrationDivider::PARENT as u32,
            multiplier: CalibrationDivider::MULTIPLIER,
            divisor: CalibrationDivider::DIVISOR,
            register_offsets: registers.map(|(offset, _)| offset),
            register_values: registers.map(|(_, value)| value),
        });
    }
}
