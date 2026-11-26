// SPDX-License-Identifier: GPL-2.0-only

use crate::{ClockId, ClockParent, ClockSource};

const MAX_PARENTS: usize = 6;
const MAX_RATES: usize = 6;

#[repr(C)]
pub(crate) struct ClockSourceData {
    command_offset_bytes: u32,
    hardware_clock_control: bool,
    set_rate_parent: bool,
    hid_width: u8,
    parent_count: u8,
    rate_count: u8,
    parents: [u8; MAX_PARENTS],
    selectors: [u8; MAX_PARENTS],
    frequencies_hz: [u32; MAX_RATES],
    rate_parents: [u8; MAX_RATES],
    rate_dividers: [u8; MAX_RATES],
}

const fn parent_id(parent: ClockParent) -> u8 {
    match parent {
        ClockParent::BiTcxo => 0,
        ClockParent::BiTcxoAlwaysOn => 1,
        ClockParent::CorePllTest => 2,
        ClockParent::CrcDivider => 3,
        ClockParent::GccGpll0 => 4,
        ClockParent::GccGpll0Divider => 5,
        ClockParent::Pll0Even => 6,
        ClockParent::Pll1Even => 7,
        ClockParent::Q6ssPllMain => 8,
    }
}

#[unsafe(no_mangle)]
extern "C" fn sm8250_npucc_clock_source(index: u32, output: *mut ClockSourceData) -> bool {
    let Some(source) = ClockId::from_index(index as usize).and_then(ClockSource::for_id) else {
        return false;
    };
    if output.is_null() || source.parents.len() > MAX_PARENTS || source.rates.len() > MAX_RATES {
        return false;
    }

    let mut data = ClockSourceData {
        command_offset_bytes: source.command_offset_bytes,
        hardware_clock_control: source.hardware_clock_control,
        set_rate_parent: source.set_rate_parent,
        hid_width: ClockSource::HID_WIDTH,
        parent_count: source.parents.len() as u8,
        rate_count: source.rates.len() as u8,
        parents: [0; MAX_PARENTS],
        selectors: [0; MAX_PARENTS],
        frequencies_hz: [0; MAX_RATES],
        rate_parents: [0; MAX_RATES],
        rate_dividers: [0; MAX_RATES],
    };
    for (index, &(parent, selector)) in source.parents.iter().enumerate() {
        data.parents[index] = parent_id(parent);
        data.selectors[index] = selector;
    }
    for (index, &rate) in source.rates.iter().enumerate() {
        data.frequencies_hz[index] = rate.frequency_hz;
        data.rate_parents[index] = parent_id(rate.parent);
        data.rate_dividers[index] = rate.hid_value();
    }

    // SAFETY: The C caller provides writable, aligned storage for one descriptor.
    unsafe { output.write(data) };
    true
}
