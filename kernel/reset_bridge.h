/* SPDX-License-Identifier: GPL-2.0-only */

#ifndef SM8250_NPU_RESET_BRIDGE_H
#define SM8250_NPU_RESET_BRIDGE_H

struct device;
struct reset_control;

struct reset_control *sm8250_npu_reset_get(struct device *dev, const char *name);

#endif
