// SPDX-License-Identifier: GPL-2.0-only

#include <linux/device.h>
#include <linux/err.h>
#include <linux/regmap.h>
#include <linux/slab.h>
#include <linux/string.h>

#include "clk-branch.h"
#include "clock_branch_bridge.h"

struct clk_regmap *sm8250_npucc_make_branch(struct device *dev, u32 index,
					   const char *name)
{
	static const u8 halt_checks[] = {
		BRANCH_HALT,
		BRANCH_HALT_DELAY,
		BRANCH_HALT_VOTED,
	};
	struct sm8250_npucc_branch_data data;
	struct clk_init_data *init;
	struct clk_branch *branch;
	struct clk_parent_data *parent;
	char *parent_name;

	if (!sm8250_npucc_clock_branch(index, &data))
		return ERR_PTR(-ENOENT);
	if (data.halt_policy >= ARRAY_SIZE(halt_checks))
		return ERR_PTR(-EINVAL);

	branch = devm_kzalloc(dev, sizeof(*branch), GFP_KERNEL);
	init = devm_kzalloc(dev, sizeof(*init), GFP_KERNEL);
	if (!branch || !init)
		return ERR_PTR(-ENOMEM);

	init->name = name;
	init->ops = &clk_branch2_ops;
	if (data.set_rate_parent)
		init->flags |= CLK_SET_RATE_PARENT;

	if (data.parent_name_len) {
		parent = devm_kzalloc(dev, sizeof(*parent), GFP_KERNEL);
		if (!parent)
			return ERR_PTR(-ENOMEM);
		parent_name = devm_kzalloc(dev, data.parent_name_len + 1, GFP_KERNEL);
		if (!parent_name)
			return ERR_PTR(-ENOMEM);
		memcpy(parent_name, data.parent_name, data.parent_name_len);
		parent->name = parent_name;
		parent->index = -1;
		init->parent_data = parent;
		init->num_parents = 1;
	}

	branch->halt_reg = data.register_offset_bytes;
	branch->halt_check = halt_checks[data.halt_policy];
	branch->clkr.enable_reg = data.register_offset_bytes;
	branch->clkr.enable_mask = BIT(0);
	branch->clkr.hw.init = init;
	return &branch->clkr;
}
