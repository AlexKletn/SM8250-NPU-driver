// SPDX-License-Identifier: GPL-2.0-only

use core::{
    ffi::{c_int, c_void},
    ptr::NonNull,
};

use kernel::error::{Error, Result, code::EINVAL, to_result};

use crate::{HmPair, HmPairOps};

unsafe extern "C" {
    fn sm8250_hm_enable_primary(hw: *mut c_void) -> c_int;
    fn sm8250_hm_enable_companion(hw: *mut c_void) -> c_int;
    fn sm8250_hm_disable_primary(hw: *mut c_void);
    fn sm8250_hm_disable_companion(hw: *mut c_void);
    fn sm8250_hm_companion_parent(hw: *mut c_void, index: u8) -> *mut c_void;
    fn sm8250_hm_set_primary_parent(hw: *mut c_void, index: u8) -> c_int;
    fn sm8250_hm_set_companion_parent(hw: *mut c_void, parent: *mut c_void) -> c_int;
    fn sm8250_hm_determine_primary_rate(hw: *mut c_void, request: *mut c_void) -> c_int;
    fn sm8250_hm_determine_companion_rate(hw: *mut c_void, request: *mut c_void) -> c_int;
    fn sm8250_hm_set_primary_rate(
        hw: *mut c_void,
        rate: usize,
        parent_rate: usize,
        index: c_int,
    ) -> c_int;
    fn sm8250_hm_set_companion_rate(hw: *mut c_void, rate: usize) -> c_int;
}

struct LinuxHmPair(NonNull<c_void>);

impl HmPairOps for LinuxHmPair {
    type Error = Error;
    type Parent = NonNull<c_void>;
    type RateRequest = NonNull<c_void>;

    fn enable_primary(&mut self) -> Result {
        // SAFETY: The CCF callback holds both clocks live and allows enabling them.
        to_result(unsafe { sm8250_hm_enable_primary(self.0.as_ptr()) })
    }

    fn enable_companion(&mut self) -> Result {
        // SAFETY: The companion belongs to the clock pair held by the CCF callback.
        to_result(unsafe { sm8250_hm_enable_companion(self.0.as_ptr()) })
    }

    fn disable_primary(&mut self) {
        // SAFETY: The CCF callback holds the primary clock and it was enabled.
        unsafe { sm8250_hm_disable_primary(self.0.as_ptr()) };
    }

    fn disable_companion(&mut self) {
        // SAFETY: The CCF callback holds the companion clock and it was enabled.
        unsafe { sm8250_hm_disable_companion(self.0.as_ptr()) };
    }

    fn companion_parent(&mut self, index: u8) -> Result<Self::Parent> {
        // SAFETY: The clock pair and its parent list belong to the active CCF callback.
        NonNull::new(unsafe { sm8250_hm_companion_parent(self.0.as_ptr(), index) }).ok_or(EINVAL)
    }

    fn set_primary_parent(&mut self, index: u8) -> Result {
        // SAFETY: The CCF callback permits setting the parent at this index.
        to_result(unsafe { sm8250_hm_set_primary_parent(self.0.as_ptr(), index) })
    }

    fn set_companion_parent(&mut self, parent: Self::Parent) -> Result {
        // SAFETY: The parent was resolved for the companion during this callback.
        to_result(unsafe { sm8250_hm_set_companion_parent(self.0.as_ptr(), parent.as_ptr()) })
    }

    fn determine_primary_rate(&mut self, request: &mut Self::RateRequest) -> Result {
        // SAFETY: The active CCF callback owns the writable rate request.
        to_result(unsafe { sm8250_hm_determine_primary_rate(self.0.as_ptr(), request.as_ptr()) })
    }

    fn determine_companion_rate(&mut self, request: &mut Self::RateRequest) -> Result {
        // SAFETY: The active CCF callback owns the writable rate request.
        to_result(unsafe { sm8250_hm_determine_companion_rate(self.0.as_ptr(), request.as_ptr()) })
    }

    fn set_primary_rate(&mut self, rate: usize, parent_rate: usize, index: Option<u8>) -> Result {
        // SAFETY: The primary is live and the CCF callback permits setting its rate.
        to_result(unsafe {
            sm8250_hm_set_primary_rate(
                self.0.as_ptr(),
                rate,
                parent_rate,
                index.map_or(-1, c_int::from),
            )
        })
    }

    fn set_companion_rate(&mut self, rate: usize) -> Result {
        // SAFETY: The companion is live and shares the rate-change context with the primary.
        to_result(unsafe { sm8250_hm_set_companion_rate(self.0.as_ptr(), rate) })
    }
}

impl LinuxHmPair {
    fn run(hw: *mut c_void, operation: impl FnOnce(&mut Self) -> Result) -> c_int {
        NonNull::new(hw)
            .ok_or(EINVAL)
            .and_then(|hw| operation(&mut Self(hw)))
            .err()
            .map_or(0, Error::to_errno)
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn sm8250_hm_pair_enable(hw: *mut c_void) -> c_int {
    LinuxHmPair::run(hw, HmPair::enable)
}

#[unsafe(no_mangle)]
unsafe extern "C" fn sm8250_hm_pair_disable(hw: *mut c_void) {
    if let Some(hw) = NonNull::new(hw) {
        HmPair::disable(&mut LinuxHmPair(hw));
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn sm8250_hm_pair_set_parent(hw: *mut c_void, index: u8) -> c_int {
    LinuxHmPair::run(hw, |clocks| HmPair::set_parent(clocks, index))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn sm8250_hm_pair_determine_rate(hw: *mut c_void, request: *mut c_void) -> c_int {
    LinuxHmPair::run(hw, |clocks| {
        let mut request = NonNull::new(request).ok_or(EINVAL)?;
        HmPair::determine_rate(clocks, &mut request)
    })
}

#[unsafe(no_mangle)]
unsafe extern "C" fn sm8250_hm_pair_set_rate(
    hw: *mut c_void,
    rate: usize,
    parent_rate: usize,
    index: c_int,
) -> c_int {
    LinuxHmPair::run(hw, |clocks| {
        let index = if index == -1 {
            None
        } else {
            Some(u8::try_from(index).map_err(|_| EINVAL)?)
        };
        HmPair::set_rate(clocks, rate, parent_rate, index)
    })
}
