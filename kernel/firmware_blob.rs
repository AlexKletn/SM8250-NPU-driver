// SPDX-License-Identifier: GPL-2.0-only

use core::{ptr, slice};
use kernel::{
    bindings,
    device::Device,
    error::{
        Result,
        code::{EINVAL, ENODEV},
        to_result,
    },
    prelude::CStr,
    str::CStrExt,
};

pub(crate) struct FirmwareBlob {
    blob: ptr::NonNull<bindings::firmware>,
}

impl FirmwareBlob {
    pub(crate) fn request(dev: &Device, name: &CStr) -> Result<Self> {
        let mut blob = ptr::null();
        let device = ptr::from_ref(dev).cast_mut().cast::<bindings::device>();
        // SAFETY: Device is transparent over struct device, name is NUL-terminated,
        // and blob is writable storage for the returned firmware pointer.
        to_result(unsafe {
            bindings::request_firmware_direct(&mut blob, name.as_char_ptr(), device)
        })?;
        let blob = ptr::NonNull::new(blob.cast_mut()).ok_or(ENODEV)?;
        Ok(Self { blob })
    }

    pub(crate) fn as_raw(&self) -> *const bindings::firmware {
        self.blob.as_ptr()
    }

    pub(crate) fn data(&self) -> Result<&[u8]> {
        // SAFETY: This object owns the live firmware reference.
        let blob = unsafe { self.blob.as_ref() };
        if blob.size == 0 {
            return Ok(&[]);
        }
        if blob.data.is_null() || blob.size > isize::MAX as usize {
            return Err(EINVAL);
        }
        // SAFETY: The firmware API keeps this immutable buffer alive for the reference lifetime.
        Ok(unsafe { slice::from_raw_parts(blob.data, blob.size) })
    }
}

impl Drop for FirmwareBlob {
    fn drop(&mut self) {
        // SAFETY: This object owns one reference returned by request_firmware_direct.
        unsafe { bindings::release_firmware(self.as_raw()) };
    }
}
