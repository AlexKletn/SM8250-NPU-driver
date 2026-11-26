use super::*;

#[test]
fn every_rate_uses_a_parent_supported_by_its_source() {
    for id in [
        ClockId::CalHm0ClkSrc,
        ClockId::CalHm1ClkSrc,
        ClockId::CoreClkSrc,
        ClockId::LmhClkSrc,
        ClockId::XoClkSrc,
        ClockId::DspCoreClkSrc,
    ] {
        let source = ClockSource::for_id(id).unwrap();
        assert!(!source.rates.is_empty());
        for rate in source.rates {
            assert!(
                source
                    .parents
                    .iter()
                    .any(|(parent, _)| *parent == rate.parent)
            );
            assert!(rate.hid_value() < (1 << ClockSource::HID_WIDTH));
        }
    }
}

#[test]
fn hm0_has_a_companion_and_a_distinct_register() {
    let hm0 = ClockSource::for_id(ClockId::CalHm0ClkSrc).unwrap();
    let hm1 = ClockSource::for_id(hm0.companion.unwrap()).unwrap();
    assert_eq!(hm0.command_offset_bytes, 0x1100);
    assert_eq!(hm1.command_offset_bytes, 0x1140);
    assert_eq!(hm1.companion, None);
    assert_eq!(hm0.parents, hm1.parents);
}
