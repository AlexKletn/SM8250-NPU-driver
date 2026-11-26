/* SPDX-License-Identifier: GPL-2.0-only */

#ifndef SM8250_NPUCC_PLL_BRIDGE_H
#define SM8250_NPUCC_PLL_BRIDGE_H

#include <linux/types.h>

struct clk_regmap;
struct device;
struct regmap;

struct sm8250_npucc_pll_data {
	u32 offset, l, alpha, calibration_l;
	u32 config_ctl, config_ctl_hi, config_ctl_hi1;
	u32 user_ctl, user_ctl_hi, user_ctl_hi1;
	u32 vco_min_hz, vco_max_hz;
	u8 vco_selector;
};

struct sm8250_npucc_post_divider_data {
	u32 offset, parent;
	u8 shift, width, encoding, divisor;
};

struct sm8250_npucc_calibration_data {
	u32 parent;
	u8 multiplier, divisor;
	u32 register_offsets[4];
	u32 register_values[4];
};

bool sm8250_npucc_pll_data(u32 index, struct sm8250_npucc_pll_data *output);
bool sm8250_npucc_post_divider_data(u32 index, struct sm8250_npucc_post_divider_data *output);
void sm8250_npucc_calibration_data(struct sm8250_npucc_calibration_data *output);
struct clk_regmap *sm8250_npucc_make_pll(struct device *dev, struct regmap *map,
					u32 index, const char *name);
struct clk_regmap *sm8250_npucc_make_post_divider(struct device *dev, u32 index,
						 const char *name,
						 const char *const *clock_names);
int sm8250_npucc_setup_calibration(struct device *dev, struct regmap *map,
				  const char *const *clock_names);

#endif
