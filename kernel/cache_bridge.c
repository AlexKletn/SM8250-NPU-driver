// SPDX-License-Identifier: GPL-2.0-only

#include <linux/of.h>
#include <linux/of_platform.h>
#include <linux/regmap.h>
#include <linux/soc/qcom/llcc-qcom.h>

#include "cache_bridge.h"

struct llcc_slice_desc *sm8250_npu_cache_get(void)
{
	return llcc_slice_getd(LLCC_NPU);
}

int sm8250_npu_cache_slice_id(struct llcc_slice_desc *slice)
{
	return llcc_get_slice_id(slice);
}

int sm8250_npu_cache_activate(struct llcc_slice_desc *slice)
{
	return llcc_slice_activate(slice);
}

int sm8250_npu_cache_deactivate(struct llcc_slice_desc *slice)
{
	return llcc_slice_deactivate(slice);
}

void sm8250_npu_cache_put(struct llcc_slice_desc *slice)
{
	llcc_slice_putd(slice);
}

int sm8250_npu_configure_cpc(struct device *dev, u32 offset, u32 value)
{
	struct of_phandle_args clock;
	struct platform_device *provider;
	struct regmap *regmap;
	int ret;

	ret = of_parse_phandle_with_args(dev->of_node, "clocks", "#clock-cells",
					0, &clock);
	if (ret)
		return ret;

	provider = of_find_device_by_node(clock.np);
	of_node_put(clock.np);
	if (!provider)
		return -ENODEV;

	regmap = dev_get_regmap(&provider->dev, NULL);
	ret = regmap ? regmap_write(regmap, offset, value) : -ENODEV;
	put_device(&provider->dev);

	return ret;
}
