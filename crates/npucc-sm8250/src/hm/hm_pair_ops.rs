pub trait HmPairOps {
    type Error;
    type Parent;
    type RateRequest;

    fn enable_primary(&mut self) -> Result<(), Self::Error>;
    fn enable_companion(&mut self) -> Result<(), Self::Error>;
    fn disable_primary(&mut self);
    fn disable_companion(&mut self);

    fn companion_parent(&mut self, index: u8) -> Result<Self::Parent, Self::Error>;
    fn set_primary_parent(&mut self, index: u8) -> Result<(), Self::Error>;
    fn set_companion_parent(&mut self, parent: Self::Parent) -> Result<(), Self::Error>;

    fn determine_primary_rate(
        &mut self,
        request: &mut Self::RateRequest,
    ) -> Result<(), Self::Error>;
    fn determine_companion_rate(
        &mut self,
        request: &mut Self::RateRequest,
    ) -> Result<(), Self::Error>;

    fn set_primary_rate(
        &mut self,
        rate_hz: usize,
        parent_rate_hz: usize,
        parent_index: Option<u8>,
    ) -> Result<(), Self::Error>;
    fn set_companion_rate(&mut self, rate_hz: usize) -> Result<(), Self::Error>;
}
