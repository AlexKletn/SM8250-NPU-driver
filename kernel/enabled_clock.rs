// SPDX-License-Identifier: GPL-2.0-only

use kernel::{
    clk::{Clk, Hertz},
    device::{Bound, Device},
    error::Result,
};

use crate::{boot_clock::BootClock, deasserted_reset::DeassertedReset};

pub(crate) struct EnabledClock<'a> {
    clock: Clk,
    _reset: Option<DeassertedReset<'a>>,
}

impl<'a> EnabledClock<'a> {
    pub(crate) fn acquire(dev: &'a Device<Bound>, step: &BootClock) -> Result<Self> {
        let reset = step
            .release_reset
            .then(|| DeassertedReset::acquire(dev, step.name))
            .transpose()?;
        let clock = Clk::get(dev, Some(step.name))?;
        clock.prepare_enable()?;
        let enabled = Self {
            clock,
            _reset: reset,
        };
        if step.rate_hz != 0 {
            enabled.clock.set_rate(Hertz(step.rate_hz as usize))?;
        }
        Ok(enabled)
    }
}

impl Drop for EnabledClock<'_> {
    fn drop(&mut self) {
        self.clock.disable_unprepare();
    }
}
