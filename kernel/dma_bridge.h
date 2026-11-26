/* SPDX-License-Identifier: GPL-2.0-only */

#ifndef SM8250_NPU_DMA_BRIDGE_H
#define SM8250_NPU_DMA_BRIDGE_H

#include <linux/types.h>

struct device;
struct sg_table;

int sm8250_npu_dma_init(struct device *dev);
int sm8250_npu_dma_iova(struct sg_table *table, size_t length, u32 *iova);
void sm8250_npu_dma_cpu_begin(struct device *dev, struct sg_table *table,
			     void *mapping, size_t length);
void sm8250_npu_dma_cpu_end(struct device *dev, struct sg_table *table,
			   void *mapping, size_t length, bool modified);

#endif
