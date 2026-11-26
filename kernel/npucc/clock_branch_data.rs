// SPDX-License-Identifier: GPL-2.0-only

use crate::{ClockBranch, ClockId, HaltCheck};

#[repr(C)]
pub(crate) struct ClockBranchData {
    register_offset_bytes: u32,
    parent_name: *const u8,
    parent_name_len: usize,
    halt_policy: u8,
    set_rate_parent: bool,
    critical: bool,
}

#[unsafe(no_mangle)]
extern "C" fn sm8250_npucc_clock_branch(index: u32, output: *mut ClockBranchData) -> bool {
    let Some(branch) = ClockId::from_index(index as usize).and_then(ClockBranch::for_id) else {
        return false;
    };
    let parent = branch.parent_name.unwrap_or_default();
    let data = ClockBranchData {
        register_offset_bytes: branch.register_offset_bytes,
        parent_name: parent.as_ptr(),
        parent_name_len: parent.len(),
        halt_policy: match branch.halt_check {
            HaltCheck::Poll => 0,
            HaltCheck::Delay => 1,
            HaltCheck::Voted => 2,
        },
        set_rate_parent: branch.set_rate_parent,
        critical: branch.critical,
    };

    if output.is_null() {
        return false;
    }
    // SAFETY: The C caller provides writable storage for one properly aligned descriptor.
    unsafe { output.write(data) };
    true
}
