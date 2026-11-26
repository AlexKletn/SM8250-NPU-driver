// SPDX-License-Identifier: GPL-2.0-only

#include <linux/clk.h>
#include <linux/module.h>
#include <linux/of.h>
#include <linux/platform_device.h>
#include <linux/pm_domain.h>
#include <linux/regmap.h>
#include <linux/slab.h>

#include "clock_branch_bridge.h"
#include "clock_names.h"
#include "clock_source_bridge.h"
#include "common.h"
#include "gdsc.h"
#include "hm_pair.h"
#include "pll_bridge.h"
#include "register_map.h"
#include "reset.h"

u32 sm8250_npucc_reset_offset(u32 index);
u8 sm8250_npucc_reset_bit(u32 index);
u32 sm8250_npucc_core_gdsc_offset(void);

enum { HM0_SOURCE = 5, HM1_SOURCE = 10, PLL0 = 34, RESET_COUNT = 7 };

static struct clk_regmap *make_source(struct device *dev, u32 id,
				      struct sm8250_npucc_hm_pair *pair,
				      struct clk_rcg2 *hm1)
{
	const struct clk_ops *ops = NULL;
	struct clk_rcg2 *source;
	int ret;

	if (id == HM0_SOURCE) {
		pair->companion = &hm1->clkr.hw;
		source = &pair->source;
		ops = &sm8250_npucc_hm_pair_ops;
	} else if (id == HM1_SOURCE) {
		source = hm1;
	} else {
		source = devm_kzalloc(dev, sizeof(*source), GFP_KERNEL);
		if (!source)
			return ERR_PTR(-ENOMEM);
	}

	ret = sm8250_npucc_configure_source(dev, id, clock_names[id], source, ops);
	return ret ? ERR_PTR(ret) : &source->clkr;
}

static int configure_clocks(struct device *dev, struct regmap *map,
			    struct clk_regmap **clocks)
{
	struct sm8250_npucc_hm_pair *pair;
	struct clk_rcg2 *hm1;
	unsigned int id;

	pair = devm_kzalloc(dev, sizeof(*pair), GFP_KERNEL);
	hm1 = devm_kzalloc(dev, sizeof(*hm1), GFP_KERNEL);
	if (!pair || !hm1)
		return -ENOMEM;

	for (id = 0; id < ARRAY_SIZE(clock_names); id++) {
		struct sm8250_npucc_source_data source;
		struct clk_regmap *clock;
		int ret;

		clock = sm8250_npucc_make_branch(dev, id, clock_names[id]);
		if (clock == ERR_PTR(-ENOENT)) {
			if (sm8250_npucc_clock_source(id, &source))
				clock = make_source(dev, id, pair, hm1);
			else
				clock = sm8250_npucc_make_pll(dev, map, id, clock_names[id]);
		}
		if (clock == ERR_PTR(-ENOENT))
			clock = sm8250_npucc_make_post_divider(dev, id, clock_names[id], clock_names);
		if (IS_ERR(clock))
			return PTR_ERR(clock);
		clocks[id] = clock;

		if (id == PLL0) {
			ret = sm8250_npucc_setup_calibration(dev, map, clock_names);
			if (ret)
				return ret;
		}
	}
	return 0;
}

static void drop_cx_vote(void *data)
{
	dev_pm_genpd_set_performance_state(data, 0);
}

static void disable_required_clock(void *data)
{
	clk_disable_unprepare(data);
}

static int enable_required_clocks(struct device *dev, const struct qcom_cc_desc *desc)
{
	unsigned int index;

	for (index = 0; index < desc->num_clks; index++) {
		struct sm8250_npucc_branch_data branch;
		struct clk *clock;
		int ret;

		if (!sm8250_npucc_clock_branch(index, &branch) || !branch.critical)
			continue;

		clock = desc->clks[index]->hw.clk;
		ret = clk_prepare_enable(clock);
		if (ret)
			return ret;
		ret = devm_add_action_or_reset(dev, disable_required_clock, clock);
		if (ret)
			return ret;
	}

	return 0;
}

static int sm8250_npucc_probe(struct platform_device *pdev)
{
	static const struct gdsc core_template = {
		.pd.name = "sm8250_npu_core",
		.pwrsts = PWRSTS_OFF_ON,
		.flags = RETAIN_FF_ENABLE,
	};
	struct device *dev = &pdev->dev;
	struct qcom_cc_desc *desc;
	struct qcom_reset_map *resets;
	struct gdsc *core;
	struct regmap *map;
	unsigned int i;
	u32 cx_state;
	int ret;

	if (of_property_present(dev->of_node, "qcom,cx-performance-state")) {
		ret = of_property_read_u32(dev->of_node, "qcom,cx-performance-state", &cx_state);
		if (ret)
			return dev_err_probe(dev, ret, "Invalid NPUCC CX performance state\n");

		ret = dev_pm_genpd_set_performance_state(dev, cx_state);
		if (ret)
			return dev_err_probe(dev, ret, "Cannot vote for NPUCC CX performance\n");
		ret = devm_add_action_or_reset(dev, drop_cx_vote, dev);
		if (ret)
			return ret;
	}

	map = sm8250_npucc_map_registers(pdev);
	if (IS_ERR(map))
		return PTR_ERR(map);

	desc = devm_kzalloc(dev, sizeof(*desc), GFP_KERNEL);
	core = devm_kmemdup(dev, &core_template, sizeof(*core), GFP_KERNEL);
	resets = devm_kcalloc(dev, RESET_COUNT, sizeof(*resets), GFP_KERNEL);
	if (!desc || !core || !resets)
		return -ENOMEM;
	desc->num_clks = ARRAY_SIZE(clock_names);
	desc->clks = devm_kcalloc(dev, desc->num_clks, sizeof(*desc->clks), GFP_KERNEL);
	desc->gdscs = devm_kmalloc(dev, sizeof(*desc->gdscs), GFP_KERNEL);
	if (!desc->clks || !desc->gdscs)
		return -ENOMEM;

	core->gdscr = sm8250_npucc_core_gdsc_offset();
	for (i = 0; i < RESET_COUNT; i++) {
		resets[i].reg = sm8250_npucc_reset_offset(i);
		resets[i].bit = sm8250_npucc_reset_bit(i);
	}
	desc->resets = resets;
	desc->num_resets = RESET_COUNT;
	desc->gdscs[0] = core;
	desc->num_gdscs = 1;

	ret = configure_clocks(dev, map, desc->clks);
	if (ret)
		return dev_err_probe(dev, ret, "Cannot configure SM8250 NPU clocks\n");
	ret = qcom_cc_really_probe(dev, desc, map);
	if (ret)
		return ret;
	return enable_required_clocks(dev, desc);
}

static const struct of_device_id sm8250_npucc_of_match[] = {
	{ .compatible = "qcom,sm8250-npucc" },
	{ }
};
MODULE_DEVICE_TABLE(of, sm8250_npucc_of_match);

static struct platform_driver sm8250_npucc_driver = {
	.probe = sm8250_npucc_probe,
	.driver = {
		.name = "sm8250_npucc",
		.of_match_table = sm8250_npucc_of_match,
	},
};
module_platform_driver(sm8250_npucc_driver);

MODULE_DESCRIPTION("Qualcomm SM8250 NPU clock controller");
MODULE_LICENSE("GPL v2");
