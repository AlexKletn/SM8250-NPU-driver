// SPDX-License-Identifier: GPL-2.0-only

#include <linux/device.h>
#include <linux/err.h>
#include <linux/regmap.h>
#include <linux/slab.h>

#include "clk-rcg.h"
#include "common.h"
#include "clock_source_bridge.h"

static const struct clk_parent_data source_parents[] = {
	{ .fw_name = "bi_tcxo" },
	{ .fw_name = "bi_tcxo_ao" },
	{ .name = "core_bi_pll_test_se", .index = -1 },
	{ .name = "npu_cc_crc_div", .index = -1 },
	{ .name = "gcc_npu_gpll0_clk_src", .index = -1 },
	{ .name = "gcc_npu_gpll0_div_clk_src", .index = -1 },
	{ .name = "npu_cc_pll0_out_even", .index = -1 },
	{ .name = "npu_cc_pll1_out_even", .index = -1 },
	{ .name = "npu_q6ss_pll", .index = -1 },
};

int sm8250_npucc_configure_source(struct device *dev, u32 index, const char *name,
				 struct clk_rcg2 *source, const struct clk_ops *ops)
{
	struct sm8250_npucc_source_data data;
	struct clk_parent_data *parents;
	struct clk_init_data *init;
	struct parent_map *mapping;
	struct freq_tbl *rates;
	unsigned int i;

	if (!sm8250_npucc_clock_source(index, &data))
		return -ENOENT;

	init = devm_kzalloc(dev, sizeof(*init), GFP_KERNEL);
	parents = devm_kcalloc(dev, data.parent_count, sizeof(*parents), GFP_KERNEL);
	mapping = devm_kcalloc(dev, data.parent_count, sizeof(*mapping), GFP_KERNEL);
	rates = devm_kcalloc(dev, data.rate_count + 1, sizeof(*rates), GFP_KERNEL);
	if (!init || !parents || !mapping || !rates)
		return -ENOMEM;

	for (i = 0; i < data.parent_count; i++) {
		u8 parent = data.parents[i];

		parents[i] = source_parents[parent];
		mapping[i].src = parent;
		mapping[i].cfg = data.selectors[i];
	}

	for (i = 0; i < data.rate_count; i++) {
		rates[i].freq = data.frequencies_hz[i];
		rates[i].src = data.rate_parents[i];
		rates[i].pre_div = data.rate_dividers[i];
	}

	init->name = name;
	init->ops = ops ?: &clk_rcg2_shared_no_init_park_ops;
	init->parent_data = parents;
	init->num_parents = data.parent_count;
	if (data.set_rate_parent)
		init->flags = CLK_SET_RATE_PARENT;

	source->cmd_rcgr = data.command_offset_bytes;
	source->hid_width = data.hid_width;
	source->parent_map = mapping;
	source->freq_tbl = rates;
	source->hw_clk_ctrl = data.hardware_clock_control;
	source->clkr.hw.init = init;
	return 0;
}
