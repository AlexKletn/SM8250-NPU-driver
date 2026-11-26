// SPDX-License-Identifier: GPL-2.0-only

use kernel::{
    device::{Bound, Device},
    error::Result,
    prelude::*,
};

use crate::{boot_clock::BootClock, enabled_clock::EnabledClock};

pub(crate) struct LinuxBootClocks<'a> {
    active: [Option<EnabledClock<'a>>; BootClock::SEQUENCE.len()],
}

impl<'a> LinuxBootClocks<'a> {
    pub(crate) fn enable(dev: &'a Device<Bound>) -> Result<Self> {
        let mut clocks = Self {
            active: [const { None }; BootClock::SEQUENCE.len()],
        };
        for (entry, step) in clocks.active.iter_mut().zip(&BootClock::SEQUENCE) {
            *entry = Some(EnabledClock::acquire(dev, step).inspect_err(|error| {
                dev_err!(dev, "Could not enable NPU clock {}: {error:?}\n", step.name);
            })?);
        }
        Ok(clocks)
    }
}

impl Drop for LinuxBootClocks<'_> {
    fn drop(&mut self) {
        for clock in self.active.iter_mut().rev() {
            drop(clock.take());
        }
    }
}
