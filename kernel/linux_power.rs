// SPDX-License-Identifier: GPL-2.0-only

use core::ffi::{c_int, c_void};
use core::ptr::{self, NonNull};

use kernel::{
    bindings,
    device::{Bound, Device},
    error::{Result, code::ENODEV, from_err_ptr, to_result},
    prelude::*,
    str::CStrExt,
};

use crate::{boot_clock::BootClock, enabled_clock::EnabledClock};

unsafe extern "C" {
    fn qmp_get(dev: *mut c_void) -> *mut c_void;
    fn qmp_send(qmp: *mut c_void, format: *const kernel::ffi::c_char, ...) -> c_int;
    fn qmp_put(qmp: *mut c_void);
    fn dev_pm_domain_attach_by_name(
        dev: *mut c_void,
        name: *const kernel::ffi::c_char,
    ) -> *mut c_void;
    fn dev_pm_domain_detach(dev: *mut c_void, power_off: bool);
    fn sm8250_npu_runtime_resume_and_get(dev: *mut c_void) -> c_int;
}

pub(crate) struct LinuxPower<'a> {
    dev: &'a Device<Bound>,
    qmp: NonNull<c_void>,
    voted: bool,
    ahb: Option<EnabledClock<'a>>,
    core: Option<NonNull<c_void>>,
}

impl<'a> LinuxPower<'a> {
    pub(crate) fn enable(dev: &'a Device<Bound>) -> Result<Self> {
        // SAFETY: Device is transparent over struct device and remains bound for this borrow.
        let raw_dev = ptr::from_ref(dev).cast_mut().cast::<c_void>();
        let qmp = NonNull::new(from_err_ptr(unsafe { qmp_get(raw_dev) })?).ok_or(ENODEV)?;
        let mut power = Self {
            dev,
            qmp,
            voted: false,
            ahb: None,
            core: None,
        };

        // SAFETY: qmp is held until Drop.
        Self::vote(qmp, true)?;
        power.voted = true;
        power.ahb = Some(EnabledClock::acquire(dev, &BootClock::GDSC_AHB)?);
        // SAFETY: raw_dev is valid for the lifetime of power.
        let core = unsafe { dev_pm_domain_attach_by_name(raw_dev, c"core".as_char_ptr()) };
        let core = NonNull::new(from_err_ptr(core)?).ok_or(ENODEV)?;
        // SAFETY: The attachment is live and has not yet acquired a runtime PM reference.
        if let Err(error) = to_result(unsafe { sm8250_npu_runtime_resume_and_get(core.as_ptr()) }) {
            // SAFETY: Resume failed and the attachment still belongs to this probe.
            unsafe { dev_pm_domain_detach(core.as_ptr(), true) };
            return Err(error);
        }
        power.core = Some(core);
        Ok(power)
    }

    fn vote(qmp: NonNull<c_void>, enabled: bool) -> Result {
        // SAFETY: Callers hold the QMP reference, and %d expects one C int argument.
        to_result(unsafe {
            qmp_send(
                qmp.as_ptr(),
                c"{class: bcm, res: npu_on, val: %d}".as_char_ptr(),
                c_int::from(enabled),
            )
        })
    }

    pub(crate) fn disable(mut self) -> Result {
        self.release()
    }

    fn release(&mut self) -> Result {
        // SAFETY: Each handle recorded here was acquired by enable and has not been released.
        unsafe {
            let core_result = self.core.take().map_or(Ok(()), |core| {
                let result = to_result(bindings::__pm_runtime_suspend(
                    core.as_ptr().cast(),
                    bindings::RPM_GET_PUT as c_int,
                ));
                dev_pm_domain_detach(core.as_ptr(), true);
                result
            });
            drop(self.ahb.take());
            let vote_result = if core::mem::take(&mut self.voted) {
                Self::vote(self.qmp, false)
            } else {
                Ok(())
            };
            core_result.and(vote_result)
        }
    }
}

impl Drop for LinuxPower<'_> {
    fn drop(&mut self) {
        if let Err(error) = self.release() {
            dev_err!(self.dev, "Could not release NPU power: {error:?}\n");
        }
        // SAFETY: qmp_get returned this reference, which only this object releases.
        unsafe { qmp_put(self.qmp.as_ptr()) };
    }
}
