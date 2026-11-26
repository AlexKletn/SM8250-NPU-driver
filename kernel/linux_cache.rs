// SPDX-License-Identifier: GPL-2.0-only

use core::{
    ffi::{c_int, c_void},
    ptr::{self, NonNull},
};
use kernel::{
    device::{Bound, Device},
    error::{
        Result,
        code::{EINVAL, ENODEV},
        from_err_ptr, to_result,
    },
    io::{Io, Mmio, Region},
    prelude::*,
};

unsafe extern "C" {
    fn sm8250_npu_cache_get() -> *mut c_void;
    fn sm8250_npu_cache_slice_id(slice: *mut c_void) -> c_int;
    fn sm8250_npu_cache_activate(slice: *mut c_void) -> c_int;
    fn sm8250_npu_cache_deactivate(slice: *mut c_void) -> c_int;
    fn sm8250_npu_cache_put(slice: *mut c_void);
    fn sm8250_npu_configure_cpc(dev: *mut c_void, offset: u32, value: u32) -> c_int;
}

pub(crate) struct LinuxCache<'a> {
    dev: &'a Device<Bound>,
    slice: NonNull<c_void>,
    active: bool,
}

impl<'a> LinuxCache<'a> {
    const SCID_MASK: u32 = 0x1f;
    const CACHEMAP_BANKS: [usize; 2] = [0x800, 0x830];
    const CACHEMAP_ENTRIES: usize = 10;
    const CPC_RESOURCE_CONTROL: u32 = 0x11c8;
    const CPC_AUTO_ACK: u32 = 3;

    pub(crate) fn enable(dev: &'a Device<Bound>, core: Mmio<'_, Region>) -> Result<Self> {
        // SAFETY: The LLCC API returns an owned slice reference or an error pointer.
        let slice = unsafe { sm8250_npu_cache_get() };
        let slice = NonNull::new(from_err_ptr(slice)?).ok_or(ENODEV)?;
        let mut cache = Self {
            dev,
            slice,
            active: false,
        };

        // SAFETY: The slice reference is held by cache, and dev is a bound struct device.
        let slice_id = unsafe { sm8250_npu_cache_slice_id(slice.as_ptr()) };
        to_result(slice_id)?;
        let slice_id = slice_id as u32;
        if slice_id > Self::SCID_MASK {
            return Err(EINVAL);
        }
        let value = (core.try_read32(Self::CACHEMAP_BANKS[0])? & !Self::SCID_MASK) | slice_id;
        for index in 0..Self::CACHEMAP_ENTRIES {
            for bank in Self::CACHEMAP_BANKS {
                core.try_write32(value, bank + index * size_of::<u32>())?;
            }
        }

        // SAFETY: cache owns the slice reference through activation and deactivation.
        to_result(unsafe { sm8250_npu_cache_activate(slice.as_ptr()) })?;
        cache.active = true;

        let raw_dev = ptr::from_ref(dev).cast_mut().cast::<c_void>();
        // SAFETY: Device is transparent over struct device and remains bound for this call.
        to_result(unsafe {
            sm8250_npu_configure_cpc(raw_dev, Self::CPC_RESOURCE_CONTROL, Self::CPC_AUTO_ACK)
        })?;
        Ok(cache)
    }
}

impl Drop for LinuxCache<'_> {
    fn drop(&mut self) {
        // SAFETY: This object owns the slice reference and tracks successful activation.
        unsafe {
            if self.active {
                if let Err(error) = to_result(sm8250_npu_cache_deactivate(self.slice.as_ptr())) {
                    dev_err!(self.dev, "Could not deactivate NPU LLCC slice: {error:?}\n");
                }
            }
            sm8250_npu_cache_put(self.slice.as_ptr());
        }
    }
}
