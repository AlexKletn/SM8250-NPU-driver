// SPDX-License-Identifier: GPL-2.0-only

#include <linux/pm_runtime.h>

#include "power_bridge.h"

int sm8250_npu_runtime_resume_and_get(struct device *dev)
{
	return pm_runtime_resume_and_get(dev);
}
