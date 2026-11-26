/* SPDX-License-Identifier: GPL-2.0-only */

#ifndef SM8250_NPUCC_REGISTER_MAP_H
#define SM8250_NPUCC_REGISTER_MAP_H

struct platform_device;
struct regmap;

struct regmap *sm8250_npucc_map_registers(struct platform_device *pdev);

#endif
