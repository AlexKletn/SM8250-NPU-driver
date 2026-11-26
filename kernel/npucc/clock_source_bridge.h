/* SPDX-License-Identifier: GPL-2.0-only */

#ifndef SM8250_NPUCC_CLOCK_SOURCE_BRIDGE_H
#define SM8250_NPUCC_CLOCK_SOURCE_BRIDGE_H

#include <linux/types.h>

#define SM8250_NPUCC_MAX_PARENTS 6
#define SM8250_NPUCC_MAX_RATES 6

struct clk_ops;
struct clk_rcg2;
struct device;

struct sm8250_npucc_source_data {
	u32 command_offset_bytes;
	bool hardware_clock_control;
	bool set_rate_parent;
	u8 hid_width;
	u8 parent_count;
	u8 rate_count;
	u8 parents[SM8250_NPUCC_MAX_PARENTS];
	u8 selectors[SM8250_NPUCC_MAX_PARENTS];
	u32 frequencies_hz[SM8250_NPUCC_MAX_RATES];
	u8 rate_parents[SM8250_NPUCC_MAX_RATES];
	u8 rate_dividers[SM8250_NPUCC_MAX_RATES];
};

bool sm8250_npucc_clock_source(u32 index, struct sm8250_npucc_source_data *output);
int sm8250_npucc_configure_source(struct device *dev, u32 index, const char *name,
				 struct clk_rcg2 *source, const struct clk_ops *ops);

#endif
