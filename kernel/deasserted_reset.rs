// SPDX-License-Identifier: GPL-2.0-only

use core::{
    ffi::{c_int, c_void},
    ptr::{self, NonNull},
};
use kernel::{
    device::{Bound, Device},
    error::{Result, code::ENODEV, from_err_ptr, to_result},
    prelude::*,
    str::CStrExt,
};

unsafe extern "C" {
    fn sm8250_npu_reset_get(dev: *mut c_void, name: *const kernel::ffi::c_char) -> *mut c_void;
    fn reset_control_deassert(reset: *mut c_void) -> c_int;
    fn reset_control_assert(reset: *mut c_void) -> c_int;
    fn reset_control_put(reset: *mut c_void);
}

pub(crate) struct DeassertedReset<'a> {
    dev: &'a Device<Bound>,
    reset: NonNull<c_void>,
}

impl<'a> DeassertedReset<'a> {
    pub(crate) fn acquire(dev: &'a Device<Bound>, name: &CStr) -> Result<Self> {
        let raw = ptr::from_ref(dev).cast_mut().cast::<c_void>();
        // SAFETY: Device is transparent over struct device and name is NUL-terminated.
        let reset = unsafe { sm8250_npu_reset_get(raw, name.as_char_ptr()) };
        let reset = NonNull::new(from_err_ptr(reset)?).ok_or(ENODEV)?;
        // SAFETY: The exclusive reset handle was just acquired.
        if let Err(error) = to_result(unsafe { reset_control_deassert(reset.as_ptr()) }) {
            // SAFETY: The acquired reference has not been put.
            unsafe { reset_control_put(reset.as_ptr()) };
            return Err(error);
        }
        Ok(Self { dev, reset })
    }
}

impl Drop for DeassertedReset<'_> {
    fn drop(&mut self) {
        // SAFETY: This object owns one acquired and deasserted reset handle.
        if let Err(error) = to_result(unsafe { reset_control_assert(self.reset.as_ptr()) }) {
            dev_err!(self.dev, "Could not restore NPU reset: {error:?}\n");
        }
        // SAFETY: This object owns the acquired reference even if asserting the reset failed.
        unsafe { reset_control_put(self.reset.as_ptr()) };
    }
}
