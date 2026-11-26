use super::*;

#[test]
fn xo_is_critical_and_uses_the_always_on_source() {
    let xo = ClockBranch::for_id(ClockId::XoClk).unwrap();
    assert_eq!(xo.register_offset_bytes, 0x1410);
    assert!(xo.critical);
    assert!(xo.set_rate_parent);
    assert_eq!(xo.parent_name, Some("npu_cc_xo_clk_src"));
}

#[test]
fn branch_halt_modes_are_kept_distinct() {
    let branch = |id| ClockBranch::for_id(id).unwrap().halt_check;
    assert_eq!(branch(ClockId::CoreClk), HaltCheck::Poll);
    assert_eq!(branch(ClockId::DspAhbsClk), HaltCheck::Voted);
    assert_eq!(branch(ClockId::LlmTempClk), HaltCheck::Delay);
    assert_eq!(ClockBranch::for_id(ClockId::CoreClkSrc), None);
}
