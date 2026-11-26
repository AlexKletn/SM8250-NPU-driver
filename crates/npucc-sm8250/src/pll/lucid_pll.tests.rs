use super::*;
use crate::PostDivider;

#[test]
fn post_dividers_reference_their_own_pll_registers() {
    for (output_id, parent_id, offset) in [
        (ClockId::Pll0OutEven, ClockId::Pll0, 0),
        (ClockId::Pll1OutEven, ClockId::Pll1, 0x400),
    ] {
        let divider = PostDivider::for_id(output_id).unwrap();
        assert_eq!(divider.parent, parent_id);
        assert_eq!(divider.pll.register_offset_bytes, offset);
        assert_eq!(LucidPll::for_id(output_id), None);
    }
}

#[test]
fn each_pll_starts_within_the_lucid_vco_range() {
    for (id, expected_hz) in [
        (ClockId::Pll0, 600_000_000),
        (ClockId::Pll1, 1_500_000_000),
        (ClockId::Q6SsPll, 300_000_000),
    ] {
        let pll = LucidPll::for_id(id).unwrap();
        let reference_hz = 19_200_000_u64;
        let fractional_hz = (u64::from(pll.alpha) * reference_hz) >> 16;
        let rate_hz = u64::from(pll.l) * reference_hz + fractional_hz;
        assert_eq!(rate_hz, expected_hz);
        assert!(
            (u64::from(LucidPll::VCO_MIN_HZ)..=u64::from(LucidPll::VCO_MAX_HZ)).contains(&rate_hz)
        );
    }
}
