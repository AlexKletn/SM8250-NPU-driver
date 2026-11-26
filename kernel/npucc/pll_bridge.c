// SPDX-License-Identifier: GPL-2.0-only

#include <linux/device.h>
#include <linux/err.h>
#include <linux/regmap.h>
#include <linux/slab.h>

#include "clk-alpha-pll.h"
#include "pll_bridge.h"

struct clk_regmap *sm8250_npucc_make_pll(struct device *dev, struct regmap *map,
					u32 index, const char *name)
{
	static const struct clk_parent_data parent = { .fw_name = "bi_tcxo" };
	struct sm8250_npucc_pll_data data;
	struct alpha_pll_config *config;
	struct clk_init_data *init;
	struct clk_alpha_pll *pll;
	struct pll_vco *vco;

	if (!sm8250_npucc_pll_data(index, &data))
		return ERR_PTR(-ENOENT);

	pll = devm_kzalloc(dev, sizeof(*pll), GFP_KERNEL);
	vco = devm_kzalloc(dev, sizeof(*vco), GFP_KERNEL);
	config = devm_kzalloc(dev, sizeof(*config), GFP_KERNEL);
	init = devm_kzalloc(dev, sizeof(*init), GFP_KERNEL);
	if (!pll || !vco || !config || !init)
		return ERR_PTR(-ENOMEM);

	vco->min_freq = data.vco_min_hz;
	vco->max_freq = data.vco_max_hz;
	vco->val = data.vco_selector;
	config->l = data.l;
	config->alpha = data.alpha;
	config->cal_l = data.calibration_l;
	config->config_ctl_val = data.config_ctl;
	config->config_ctl_hi_val = data.config_ctl_hi;
	config->config_ctl_hi1_val = data.config_ctl_hi1;
	config->user_ctl_val = data.user_ctl;
	config->user_ctl_hi_val = data.user_ctl_hi;
	config->user_ctl_hi1_val = data.user_ctl_hi1;

	init->name = name;
	init->ops = &clk_alpha_pll_lucid_ops;
	init->parent_data = &parent;
	init->num_parents = 1;

	pll->offset = data.offset;
	pll->vco_table = vco;
	pll->num_vco = 1;
	pll->regs = clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_LUCID];
	pll->config = config;
	pll->clkr.hw.init = init;
	clk_lucid_pll_configure(pll, map, config);
	return &pll->clkr;
}

struct clk_regmap *sm8250_npucc_make_post_divider(struct device *dev, u32 index,
						 const char *name,
						 const char *const *clock_names)
{
	struct sm8250_npucc_post_divider_data data;
	struct clk_alpha_pll_postdiv *divider;
	struct clk_init_data *init;
	struct clk_div_table *table;

	if (!sm8250_npucc_post_divider_data(index, &data))
		return ERR_PTR(-ENOENT);

	divider = devm_kzalloc(dev, sizeof(*divider), GFP_KERNEL);
	init = devm_kzalloc(dev, sizeof(*init), GFP_KERNEL);
	table = devm_kcalloc(dev, 2, sizeof(*table), GFP_KERNEL);
	if (!divider || !init || !table)
		return ERR_PTR(-ENOMEM);

	table[0].val = data.encoding;
	table[0].div = data.divisor;
	init->name = name;
	init->ops = &clk_alpha_pll_postdiv_lucid_ops;
	init->flags = CLK_SET_RATE_PARENT;
	init->parent_names = &clock_names[data.parent];
	init->num_parents = 1;

	divider->offset = data.offset;
	divider->post_div_shift = data.shift;
	divider->post_div_table = table;
	divider->num_post_div = 2;
	divider->width = data.width;
	divider->regs = clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_LUCID];
	divider->clkr.hw.init = init;
	return &divider->clkr;
}

int sm8250_npucc_setup_calibration(struct device *dev, struct regmap *map,
				  const char *const *clock_names)
{
	struct sm8250_npucc_calibration_data data;
	struct clk_hw *divider;
	unsigned int i;
	int ret;

	sm8250_npucc_calibration_data(&data);
	for (i = 0; i < ARRAY_SIZE(data.register_offsets); i++) {
		ret = regmap_write(map, data.register_offsets[i], data.register_values[i]);
		if (ret)
			return ret;
	}

	divider = devm_clk_hw_register_fixed_factor(dev, "npu_cc_crc_div",
						   clock_names[data.parent],
						   CLK_SET_RATE_PARENT,
						   data.multiplier,
						   data.divisor);
	return PTR_ERR_OR_ZERO(divider);
}
