use super::HmPairOps;

pub struct HmPair;

impl HmPair {
    pub fn enable<O: HmPairOps>(clocks: &mut O) -> Result<(), O::Error> {
        clocks.enable_primary()?;
        if let Err(error) = clocks.enable_companion() {
            clocks.disable_primary();
            return Err(error);
        }
        Ok(())
    }

    pub fn disable(clocks: &mut impl HmPairOps) {
        clocks.disable_primary();
        clocks.disable_companion();
    }

    pub fn set_parent<O: HmPairOps>(clocks: &mut O, index: u8) -> Result<(), O::Error> {
        let parent = clocks.companion_parent(index)?;
        clocks.set_primary_parent(index)?;
        clocks.set_companion_parent(parent)
    }

    pub fn determine_rate<O: HmPairOps>(
        clocks: &mut O,
        request: &mut O::RateRequest,
    ) -> Result<(), O::Error> {
        clocks.determine_companion_rate(request)?;
        clocks.determine_primary_rate(request)
    }

    pub fn set_rate<O: HmPairOps>(
        clocks: &mut O,
        rate_hz: usize,
        parent_rate_hz: usize,
        parent_index: Option<u8>,
    ) -> Result<(), O::Error> {
        clocks.set_primary_rate(rate_hz, parent_rate_hz, parent_index)?;
        clocks.set_companion_rate(rate_hz)
    }
}

#[cfg(test)]
#[path = "hm_pair.tests.rs"]
mod tests;
