// SPDX-License-Identifier: GPL-2.0-only

#include <linux/err.h>
#include <linux/io.h>
#include <linux/platform_device.h>
#include <linux/regmap.h>
#include <linux/slab.h>

#include "register_map.h"

static const unsigned int bank_last[] = { 0xa060, 0x203c, 0x50 };

struct npucc_registers {
	void __iomem *bank[ARRAY_SIZE(bank_last)];
};

static bool register_valid(struct device *dev, unsigned int offset)
{
	unsigned int bank = offset >> 16;

	return bank < ARRAY_SIZE(bank_last) && (offset & 0xffff) <= bank_last[bank];
}

static int bank_read(void *context, unsigned int offset, unsigned int *value)
{
	struct npucc_registers *registers = context;

	*value = readl_relaxed(registers->bank[offset >> 16] + (offset & 0xffff));
	return 0;
}

static int bank_write(void *context, unsigned int offset, unsigned int value)
{
	struct npucc_registers *registers = context;

	writel_relaxed(value, registers->bank[offset >> 16] + (offset & 0xffff));
	return 0;
}

struct regmap *sm8250_npucc_map_registers(struct platform_device *pdev)
{
	static const char *const names[] = { "cc", "qdsp6ss", "qdsp6ss_pll" };
	static const struct regmap_config config = {
		.reg_bits = 32,
		.val_bits = 32,
		.reg_stride = 4,
		.max_register = 0x20050,
		.fast_io = true,
		.readable_reg = register_valid,
		.writeable_reg = register_valid,
		.reg_read = bank_read,
		.reg_write = bank_write,
	};
	struct device *dev = &pdev->dev;
	struct npucc_registers *registers;
	unsigned int i;

	registers = devm_kzalloc(dev, sizeof(*registers), GFP_KERNEL);
	if (!registers)
		return ERR_PTR(-ENOMEM);

	for (i = 0; i < ARRAY_SIZE(names); i++) {
		struct resource *resource;

		resource = platform_get_resource_byname(pdev, IORESOURCE_MEM, names[i]);
		if (!resource || resource_size(resource) < bank_last[i] + sizeof(u32))
			return ERR_PTR(-EINVAL);
		registers->bank[i] = devm_ioremap_resource(dev, resource);
		if (IS_ERR(registers->bank[i]))
			return ERR_CAST(registers->bank[i]);
	}

	return devm_regmap_init(dev, NULL, registers, &config);
}
