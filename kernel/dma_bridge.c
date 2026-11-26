// SPDX-License-Identifier: GPL-2.0-only

#include <linux/dma-mapping.h>
#include <linux/highmem.h>
#include <linux/iommu.h>
#include <linux/overflow.h>

#include "dma_bridge.h"

int sm8250_npu_dma_init(struct device *dev)
{
	struct iommu_domain *domain = iommu_get_domain_for_dev(dev);

	if (!domain || !iommu_is_dma_domain(domain))
		return -ENODEV;

	return dma_set_mask_and_coherent(dev, DMA_BIT_MASK(32));
}

int sm8250_npu_dma_iova(struct sg_table *table, size_t length, u32 *iova)
{
	u64 last;

	if (table->nents != 1 || sg_dma_len(table->sgl) < length ||
	    check_add_overflow((u64)sg_dma_address(table->sgl), length - 1, &last) ||
	    last > U32_MAX)
		return -ERANGE;

	*iova = sg_dma_address(table->sgl);
	return 0;
}

void sm8250_npu_dma_cpu_begin(struct device *dev, struct sg_table *table,
			     void *mapping, size_t length)
{
	dma_sync_sgtable_for_cpu(dev, table, DMA_BIDIRECTIONAL);
	invalidate_kernel_vmap_range(mapping, length);
}

void sm8250_npu_dma_cpu_end(struct device *dev, struct sg_table *table,
			   void *mapping, size_t length, bool modified)
{
	if (modified)
		flush_kernel_vmap_range(mapping, length);
	dma_sync_sgtable_for_device(dev, table, DMA_BIDIRECTIONAL);
}
