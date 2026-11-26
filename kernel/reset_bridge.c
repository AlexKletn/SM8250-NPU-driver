// SPDX-License-Identifier: GPL-2.0-only

#include <linux/reset.h>

#include "reset_bridge.h"

struct reset_control *sm8250_npu_reset_get(struct device *dev, const char *name)
{
	return reset_control_get_exclusive(dev, name);
}
