/* SPDX-License-Identifier: GPL-2.0-only */

#ifndef SM8250_NPU_CACHE_BRIDGE_H
#define SM8250_NPU_CACHE_BRIDGE_H

#include <linux/types.h>

struct device;
struct llcc_slice_desc;

struct llcc_slice_desc *sm8250_npu_cache_get(void);
int sm8250_npu_cache_slice_id(struct llcc_slice_desc *slice);
int sm8250_npu_cache_activate(struct llcc_slice_desc *slice);
int sm8250_npu_cache_deactivate(struct llcc_slice_desc *slice);
void sm8250_npu_cache_put(struct llcc_slice_desc *slice);
int sm8250_npu_configure_cpc(struct device *dev, u32 offset, u32 value);

#endif
