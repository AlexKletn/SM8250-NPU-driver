// SPDX-License-Identifier: GPL-2.0-only

#include <linux/clk.h>

#include "hm_pair.h"

static struct clk_hw *companion(struct clk_hw *hw)
{
	return container_of(to_clk_rcg2(hw), struct sm8250_npucc_hm_pair, source)->companion;
}

static int pair_prepare(struct clk_hw *hw)
{
	return clk_prepare(companion(hw)->clk);
}

static void pair_unprepare(struct clk_hw *hw)
{
	clk_unprepare(companion(hw)->clk);
}

int sm8250_hm_enable_primary(struct clk_hw *hw)
{
	return clk_rcg2_shared_ops.enable(hw);
}

int sm8250_hm_enable_companion(struct clk_hw *hw)
{
	return clk_enable(companion(hw)->clk);
}

void sm8250_hm_disable_primary(struct clk_hw *hw)
{
	clk_rcg2_shared_ops.disable(hw);
}

void sm8250_hm_disable_companion(struct clk_hw *hw)
{
	clk_disable(companion(hw)->clk);
}

static u8 pair_get_parent(struct clk_hw *hw)
{
	return clk_rcg2_shared_ops.get_parent(hw);
}

struct clk_hw *sm8250_hm_companion_parent(struct clk_hw *hw, u8 index)
{
	return clk_hw_get_parent_by_index(companion(hw), index);
}

int sm8250_hm_set_primary_parent(struct clk_hw *hw, u8 index)
{
	return clk_rcg2_shared_ops.set_parent(hw, index);
}

int sm8250_hm_set_companion_parent(struct clk_hw *hw, struct clk_hw *parent)
{
	return clk_set_parent(companion(hw)->clk, parent->clk);
}

static unsigned long pair_recalc_rate(struct clk_hw *hw, unsigned long parent_rate)
{
	return clk_rcg2_shared_ops.recalc_rate(hw, parent_rate);
}

int sm8250_hm_determine_primary_rate(struct clk_hw *hw,
				    struct clk_rate_request *request)
{
	return clk_rcg2_shared_ops.determine_rate(hw, request);
}

int sm8250_hm_determine_companion_rate(struct clk_hw *hw,
				      struct clk_rate_request *request)
{
	return __clk_determine_rate(companion(hw), request);
}

int sm8250_hm_set_primary_rate(struct clk_hw *hw, unsigned long rate,
			       unsigned long parent_rate, int index)
{
	if (index < 0)
		return clk_rcg2_shared_ops.set_rate(hw, rate, parent_rate);

	return clk_rcg2_shared_ops.set_rate_and_parent(hw, rate, parent_rate, index);
}

int sm8250_hm_set_companion_rate(struct clk_hw *hw, unsigned long rate)
{
	return clk_set_rate(companion(hw)->clk, rate);
}

static int pair_set_rate(struct clk_hw *hw, unsigned long rate,
			 unsigned long parent_rate)
{
	return sm8250_hm_pair_set_rate(hw, rate, parent_rate, -1);
}

static int pair_set_rate_and_parent(struct clk_hw *hw, unsigned long rate,
				    unsigned long parent_rate, u8 index)
{
	return sm8250_hm_pair_set_rate(hw, rate, parent_rate, index);
}

static int pair_init(struct clk_hw *hw)
{
	return clk_rcg2_shared_ops.init(hw);
}

const struct clk_ops sm8250_npucc_hm_pair_ops = {
	.prepare = pair_prepare,
	.unprepare = pair_unprepare,
	.enable = sm8250_hm_pair_enable,
	.disable = sm8250_hm_pair_disable,
	.get_parent = pair_get_parent,
	.set_parent = sm8250_hm_pair_set_parent,
	.recalc_rate = pair_recalc_rate,
	.determine_rate = sm8250_hm_pair_determine_rate,
	.set_rate = pair_set_rate,
	.set_rate_and_parent = pair_set_rate_and_parent,
	.init = pair_init,
};
