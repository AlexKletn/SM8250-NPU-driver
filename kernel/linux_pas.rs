// SPDX-License-Identifier: GPL-2.0-only

use core::{
    ffi::{c_int, c_void},
    marker::PhantomData,
    ptr::{self, NonNull},
};
use kernel::{
    device::{Bound, Device},
    error::{Result, code::ENODEV, from_err_ptr, to_result},
    prelude::CStr,
    str::CStrExt,
};

use crate::firmware_blob::FirmwareBlob;

unsafe extern "C" {
    fn sm8250_npu_pas_load(
        dev: *mut c_void,
        name: *const kernel::ffi::c_char,
        image: *const c_void,
    ) -> *mut c_void;
    fn sm8250_npu_pas_start(context: *mut c_void) -> c_int;
    fn sm8250_npu_pas_stop() -> c_int;
    fn sm8250_npu_pas_release(context: *mut c_void);
    fn sm8250_npu_pin_module();
}

pub(crate) struct LinuxPas<'a> {
    context: NonNull<c_void>,
    _device: PhantomData<&'a Device<Bound>>,
}

impl<'a> LinuxPas<'a> {
    pub(crate) fn load(dev: &'a Device<Bound>, name: &CStr) -> Result<Self> {
        let image = FirmwareBlob::request(dev, name)?;
        let raw = ptr::from_ref(dev).cast_mut().cast::<c_void>();
        // SAFETY: The bound device outlives the PAS context and name is NUL-terminated. The image is borrowed for the duration of the call.
        let context =
            unsafe { sm8250_npu_pas_load(raw, name.as_char_ptr(), image.as_raw().cast()) };
        let context = NonNull::new(from_err_ptr(context)?).ok_or(ENODEV)?;
        Ok(Self {
            context,
            _device: PhantomData,
        })
    }

    pub(crate) fn start(&mut self) -> Result {
        // SAFETY: The context belongs to this object and its image has been loaded.
        to_result(unsafe { sm8250_npu_pas_start(self.context.as_ptr()) })
    }

    pub(crate) fn stop(&mut self) -> Result {
        // SAFETY: The driver holds the powered accelerator and has started this PAS instance.
        to_result(unsafe { sm8250_npu_pas_stop() })
    }

    pub(crate) fn pin_module_after_failed_stop() {
        // SAFETY: This is called from the module's probe while its registration is live.
        unsafe { sm8250_npu_pin_module() };
    }
}

impl Drop for LinuxPas<'_> {
    fn drop(&mut self) {
        // SAFETY: The PAS context is still live and this object owns its metadata.
        unsafe { sm8250_npu_pas_release(self.context.as_ptr()) };
    }
}
