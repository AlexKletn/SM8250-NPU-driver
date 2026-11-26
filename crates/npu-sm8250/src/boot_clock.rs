// SPDX-License-Identifier: GPL-2.0-only

use core::ffi::CStr;

pub struct BootClock {
    pub name: &'static CStr,
    pub rate_hz: u32,
    pub release_reset: bool,
}

impl BootClock {
    const fn new(name: &'static CStr, rate_hz: u32, release_reset: bool) -> Self {
        Self {
            name,
            rate_hz,
            release_reset,
        }
    }

    pub const GDSC_AHB: Self = Self::new(c"gdsc_ahb", 0, false);

    pub const SEQUENCE: [Self; 28] = [
        Self::new(c"xo_clk", 0, false),
        Self::new(c"npu_core_clk", 100_000_000, false),
        Self::new(c"cal_hm0_clk", 300_000_000, false),
        Self::new(c"cal_hm1_clk", 300_000_000, false),
        Self::new(c"cal_hm0_cdc_clk", 300_000_000, false),
        Self::new(c"cal_hm1_cdc_clk", 300_000_000, false),
        Self::new(c"axi_clk", 0, false),
        Self::new(c"ahb_clk", 0, false),
        Self::new(c"dma_clk", 0, false),
        Self::new(c"llm_clk", 100_000_000, false),
        Self::new(c"llm_xo_clk", 0, false),
        Self::new(c"llm_temp_clk", 50_000_000, true),
        Self::new(c"llm_curr_clk", 50_000_000, true),
        Self::new(c"dl_llm_clk", 100_000_000, false),
        Self::new(c"isense_clk", 100_000_000, false),
        Self::new(c"dpm_clk", 100_000_000, false),
        Self::new(c"dpm_xo_clk", 0, false),
        Self::new(c"dl_dpm_clk", 0, false),
        Self::new(c"rsc_xo_clk", 0, false),
        Self::new(c"dpm_temp_clk", 0, true),
        Self::new(c"cal_hm0_dpm_ip_clk", 200_000_000, false),
        Self::new(c"cal_hm1_dpm_ip_clk", 200_000_000, false),
        Self::new(c"s2p_clk", 0, false),
        Self::new(c"bwmon_clk", 0, false),
        Self::new(c"cal_hm0_perf_cnt_clk", 0, false),
        Self::new(c"cal_hm1_perf_cnt_clk", 0, false),
        Self::new(c"bto_core_clk", 0, false),
        Self::new(c"dsp_core_clk_src", 300_000_000, false),
    ];
}
