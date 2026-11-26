// SPDX-License-Identifier: GPL-2.0-only

#include <linux/firmware/qcom/qcom_pas.h>
#include <linux/module.h>
#include <linux/of_reserved_mem.h>
#include <linux/soc/qcom/mdt_loader.h>

#include "pas_bridge.h"

#define SM8250_NPU_PAS_ID 0x17

struct qcom_pas_context *sm8250_npu_pas_load(struct device *dev, const char *name,
						  const struct firmware *image)
{
	struct qcom_pas_context *context;
	struct resource memory;
	int ret;

	ret = of_reserved_mem_region_to_resource(dev->of_node, 0, &memory);
	if (ret)
		return ERR_PTR(ret);

	context = devm_qcom_pas_context_alloc(dev, SM8250_NPU_PAS_ID,
					     memory.start, resource_size(&memory));
	if (IS_ERR(context))
		return context;

	ret = qcom_mdt_pas_load(context, image, name, NULL);
	if (ret) {
		qcom_pas_metadata_release(context);
		return ERR_PTR(ret);
	}

	return context;
}

int sm8250_npu_pas_start(struct qcom_pas_context *context)
{
	return qcom_pas_prepare_and_auth_reset(context);
}

int sm8250_npu_pas_stop(void)
{
	return qcom_pas_shutdown(SM8250_NPU_PAS_ID);
}

void sm8250_npu_pas_release(struct qcom_pas_context *context)
{
	qcom_pas_metadata_release(context);
}

void sm8250_npu_pin_module(void)
{
	__module_get(THIS_MODULE);
}
