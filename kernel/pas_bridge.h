/* SPDX-License-Identifier: GPL-2.0-only */

#ifndef SM8250_NPU_PAS_BRIDGE_H
#define SM8250_NPU_PAS_BRIDGE_H

struct device;
struct firmware;
struct qcom_pas_context;

struct qcom_pas_context *sm8250_npu_pas_load(struct device *dev, const char *name,
						  const struct firmware *image);
int sm8250_npu_pas_start(struct qcom_pas_context *context);
int sm8250_npu_pas_stop(void);
void sm8250_npu_pas_release(struct qcom_pas_context *context);
void sm8250_npu_pin_module(void);

#endif
