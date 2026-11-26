/* SPDX-License-Identifier: GPL-2.0-only */

#ifndef SM8250_NPUCC_HM_PAIR_H
#define SM8250_NPUCC_HM_PAIR_H

#include "clk-rcg.h"

struct sm8250_npucc_hm_pair {
	struct clk_rcg2 source;
	struct clk_hw *companion;
};

extern const struct clk_ops sm8250_npucc_hm_pair_ops;

int sm8250_hm_pair_enable(struct clk_hw *hw);
void sm8250_hm_pair_disable(struct clk_hw *hw);
int sm8250_hm_pair_set_parent(struct clk_hw *hw, u8 index);
int sm8250_hm_pair_determine_rate(struct clk_hw *hw, struct clk_rate_request *request);
int sm8250_hm_pair_set_rate(struct clk_hw *hw, unsigned long rate,
			   unsigned long parent_rate, int index);

int sm8250_hm_enable_primary(struct clk_hw *hw);
int sm8250_hm_enable_companion(struct clk_hw *hw);
void sm8250_hm_disable_primary(struct clk_hw *hw);
void sm8250_hm_disable_companion(struct clk_hw *hw);
struct clk_hw *sm8250_hm_companion_parent(struct clk_hw *hw, u8 index);
int sm8250_hm_set_primary_parent(struct clk_hw *hw, u8 index);
int sm8250_hm_set_companion_parent(struct clk_hw *hw, struct clk_hw *parent);
int sm8250_hm_determine_primary_rate(struct clk_hw *hw, struct clk_rate_request *request);
int sm8250_hm_determine_companion_rate(struct clk_hw *hw, struct clk_rate_request *request);
int sm8250_hm_set_primary_rate(struct clk_hw *hw, unsigned long rate,
			       unsigned long parent_rate, int index);
int sm8250_hm_set_companion_rate(struct clk_hw *hw, unsigned long rate);

#endif
