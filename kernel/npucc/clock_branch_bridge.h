/* SPDX-License-Identifier: GPL-2.0-only */

#ifndef SM8250_NPUCC_CLOCK_BRANCH_BRIDGE_H
#define SM8250_NPUCC_CLOCK_BRANCH_BRIDGE_H

#include <linux/types.h>

struct clk_regmap;
struct device;
struct sm8250_npucc_branch_data {
	u32 register_offset_bytes;
	const char *parent_name;
	size_t parent_name_len;
	u8 halt_policy;
	bool set_rate_parent;
	bool critical;
};

bool sm8250_npucc_clock_branch(u32 index, struct sm8250_npucc_branch_data *output);
struct clk_regmap *sm8250_npucc_make_branch(struct device *dev, u32 index,
					   const char *name);

#endif
