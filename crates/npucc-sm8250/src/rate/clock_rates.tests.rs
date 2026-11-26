use super::*;

#[test]
fn core_clock_preserves_fractional_dividers() {
    let rates = ClockId::CoreClkSrc.source_rates();
    assert_eq!(rates[2].frequency_hz, 333_333_333);
    assert_eq!(rates[2].parent, Pll1Even);
    assert_eq!(rates[2].hid_value(), 8);
    assert_eq!(rates[3].frequency_hz, 428_571_429);
    assert_eq!(rates[3].hid_value(), 6);
}

#[test]
fn both_calibration_sources_use_the_same_rates() {
    assert_eq!(
        ClockId::CalHm0ClkSrc.source_rates(),
        ClockId::CalHm1ClkSrc.source_rates()
    );
    assert_eq!(
        ClockId::CalHm0ClkSrc
            .source_rates()
            .last()
            .unwrap()
            .frequency_hz,
        1_000_000_000
    );
}
