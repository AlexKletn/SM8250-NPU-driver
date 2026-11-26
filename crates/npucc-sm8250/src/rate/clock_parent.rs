#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClockParent {
    BiTcxo,
    BiTcxoAlwaysOn,
    CorePllTest,
    CrcDivider,
    GccGpll0,
    GccGpll0Divider,
    Pll0Even,
    Pll1Even,
    Q6ssPllMain,
}
