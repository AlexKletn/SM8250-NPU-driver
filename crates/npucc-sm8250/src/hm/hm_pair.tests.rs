use super::*;

#[derive(Default)]
struct Mock {
    steps: [u8; 4],
    count: usize,
    fail_at: u8,
}

impl Mock {
    fn record(&mut self, step: u8) -> Result<(), u8> {
        self.steps[self.count] = step;
        self.count += 1;
        if self.fail_at == step {
            Err(step)
        } else {
            Ok(())
        }
    }
}

impl HmPairOps for Mock {
    type Error = u8;
    type Parent = u8;
    type RateRequest = ();

    fn enable_primary(&mut self) -> Result<(), u8> {
        self.record(1)
    }
    fn enable_companion(&mut self) -> Result<(), u8> {
        self.record(2)
    }
    fn disable_primary(&mut self) {
        let _ = self.record(3);
    }
    fn disable_companion(&mut self) {
        let _ = self.record(4);
    }
    fn companion_parent(&mut self, index: u8) -> Result<u8, u8> {
        self.record(5)?;
        Ok(index)
    }
    fn set_primary_parent(&mut self, _: u8) -> Result<(), u8> {
        self.record(6)
    }
    fn set_companion_parent(&mut self, _: u8) -> Result<(), u8> {
        self.record(7)
    }
    fn determine_primary_rate(&mut self, _: &mut ()) -> Result<(), u8> {
        self.record(8)
    }
    fn determine_companion_rate(&mut self, _: &mut ()) -> Result<(), u8> {
        self.record(9)
    }
    fn set_primary_rate(&mut self, _: usize, _: usize, _: Option<u8>) -> Result<(), u8> {
        self.record(10)
    }
    fn set_companion_rate(&mut self, _: usize) -> Result<(), u8> {
        self.record(11)
    }
}

#[test]
fn companion_enable_failure_rolls_back_primary() {
    let mut clocks = Mock {
        fail_at: 2,
        ..Mock::default()
    };
    assert_eq!(HmPair::enable(&mut clocks), Err(2));
    assert_eq!(&clocks.steps[..clocks.count], &[1, 2, 3]);
}

#[test]
fn absent_companion_parent_does_not_change_primary() {
    let mut clocks = Mock {
        fail_at: 5,
        ..Mock::default()
    };
    assert_eq!(HmPair::set_parent(&mut clocks, 2), Err(5));
    assert_eq!(&clocks.steps[..clocks.count], &[5]);
}

#[test]
fn rate_negotiation_starts_with_the_companion() {
    let mut clocks = Mock::default();
    HmPair::determine_rate(&mut clocks, &mut ()).unwrap();
    assert_eq!(&clocks.steps[..clocks.count], &[9, 8]);
}
