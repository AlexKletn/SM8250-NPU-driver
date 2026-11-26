use super::ClockParent;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ClockRate {
    pub frequency_hz: u32,
    pub parent: ClockParent,
    divider_half_steps: u8,
}

impl ClockRate {
    pub(super) const fn new(
        frequency_hz: u32,
        parent: ClockParent,
        divider_half_steps: u8,
    ) -> Self {
        Self {
            frequency_hz,
            parent,
            divider_half_steps,
        }
    }

    pub const fn divider_half_steps(self) -> u8 {
        self.divider_half_steps
    }

    /// Qualcomm RCG2 encodes the pre-divider as twice the divisor minus one.
    pub const fn hid_value(self) -> u8 {
        self.divider_half_steps - 1
    }
}
