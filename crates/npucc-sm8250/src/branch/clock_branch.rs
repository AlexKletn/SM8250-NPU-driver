// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2018, The Linux Foundation. All rights reserved.

use crate::ClockId;

use super::{HaltCheck, HaltCheck::*};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ClockBranch {
    pub register_offset_bytes: u32,
    pub halt_check: HaltCheck,
    pub parent_name: Option<&'static str>,
    pub set_rate_parent: bool,
    pub critical: bool,
}

impl ClockBranch {
    pub const ENABLE_MASK: u32 = 1;

    pub const fn for_id(id: ClockId) -> Option<Self> {
        let (register_offset_bytes, halt_check, parent_name, set_rate_parent, critical) = match id {
            ClockId::AtbClk => (0x10d0, Poll, Some("qdss_qmp_clk"), false, false),
            ClockId::BtoCoreClk => (0x10dc, Poll, Some("npu_cc_xo_clk_src"), true, false),
            ClockId::BwmonClk => (0x10d8, Poll, Some("npu_cc_xo_clk_src"), true, false),
            ClockId::CalHm0CdcClk => (0x1098, Poll, Some("npu_cc_cal_hm0_clk_src"), true, false),
            ClockId::CalHm0Clk => (0x1110, Poll, Some("npu_cc_cal_hm0_clk_src"), true, false),
            ClockId::CalHm0DpmIpClk => (0x109c, Poll, Some("npu_cc_cal_hm0_clk_src"), true, false),
            ClockId::CalHm0PerfCntClk => {
                (0x10a0, Poll, Some("npu_cc_cal_hm0_clk_src"), true, false)
            }
            ClockId::CalHm1CdcClk => (0x10a4, Poll, Some("npu_cc_cal_hm0_clk_src"), true, false),
            ClockId::CalHm1Clk => (0x1150, Poll, Some("npu_cc_cal_hm0_clk_src"), true, false),
            ClockId::CalHm1DpmIpClk => (0x10a8, Poll, Some("npu_cc_cal_hm0_clk_src"), true, false),
            ClockId::CalHm1PerfCntClk => {
                (0x10ac, Poll, Some("npu_cc_cal_hm0_clk_src"), true, false)
            }
            ClockId::CoreClk => (0x1030, Poll, Some("npu_cc_core_clk_src"), true, false),
            ClockId::DlDpmClk => (0x1238, Poll, Some("npu_cc_lmh_clk_src"), true, false),
            ClockId::DlLlmClk => (0x1234, Poll, Some("npu_cc_lmh_clk_src"), true, false),
            ClockId::DpmClk => (0x107c, Poll, Some("npu_cc_lmh_clk_src"), true, false),
            ClockId::DpmTempClk => (0x10c4, Delay, Some("npu_llm_temp_clk"), true, false),
            ClockId::DpmXoClk => (0x1094, Poll, Some("npu_cc_xo_clk_src"), true, false),
            ClockId::DspAhbmClk => (0x1214, Poll, Some("npu_cc_core_clk_src"), true, false),
            ClockId::DspAhbsClk => (0x1210, Voted, Some("npu_cc_core_clk_src"), true, false),
            ClockId::DspAxiClk => (0x121c, Poll, Some("gcc_npu_axi_clk"), true, false),
            ClockId::DspBwmonAhbClk => (0x1218, Poll, Some("npu_cc_core_clk_src"), true, false),
            ClockId::DspBwmonClk => (0x1224, Poll, Some("npu_cc_xo_clk_src"), true, false),
            ClockId::IsenseClk => (0x1078, Poll, Some("npu_cc_lmh_clk_src"), true, false),
            ClockId::LlmClk => (0x1074, Poll, Some("npu_cc_lmh_clk_src"), true, false),
            ClockId::LlmCurrClk => (0x10d4, Delay, Some("npu_llm_curr_clk"), true, false),
            ClockId::LlmTempClk => (0x10c8, Delay, Some("npu_llm_temp_clk"), true, false),
            ClockId::LlmXoClk => (0x1090, Poll, Some("npu_cc_xo_clk_src"), true, false),
            ClockId::NocAhbClk => (0x10c0, Poll, None, false, false),
            ClockId::NocAxiClk => (0x10b8, Poll, Some("gcc_npu_axi_clk"), true, false),
            ClockId::NocDmaClk => (0x10b0, Poll, Some("gcc_npu_dma_clk"), true, false),
            ClockId::RscXoClk => (0x10e0, Poll, Some("npu_cc_xo_clk_src"), true, false),
            ClockId::S2PClk => (0x10cc, Delay, Some("npu_s2p_clk"), true, false),
            ClockId::XoClk => (0x1410, Poll, Some("npu_cc_xo_clk_src"), true, true),
            _ => return None,
        };
        Some(Self {
            register_offset_bytes,
            halt_check,
            parent_name,
            set_rate_parent,
            critical,
        })
    }
}

#[cfg(test)]
#[path = "clock_branch.tests.rs"]
mod tests;
