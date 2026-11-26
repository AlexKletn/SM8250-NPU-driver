// SPDX-License-Identifier: GPL-2.0-only

//! Linux platform driver for the SM8250 NPU.

#[allow(missing_docs)]
#[path = "../crates/npu-sm8250/src/ipc/mod.rs"]
pub mod ipc;

#[allow(missing_docs)]
#[path = "../crates/npu-sm8250/src/firmware/mod.rs"]
pub mod firmware;

#[allow(missing_docs)]
#[path = "../crates/npu-sm8250/src/boot_clock.rs"]
pub mod boot_clock;

#[allow(missing_docs)]
#[path = "../crates/npu-domain/src/graph_relocation_error.rs"]
mod graph_relocation_error;
#[allow(missing_docs)]
#[path = "../crates/npu-domain/src/graph_relocations.rs"]
mod graph_relocations;
pub use graph_relocation_error::GraphRelocationError;
pub use graph_relocations::GraphRelocations;

mod accelerator_device;
mod apps_exchange;
mod core_mmio;
mod deasserted_reset;
mod enabled_clock;
mod firmware_blob;
mod firmware_exchange;
mod graph_exchange;
mod graph_validation;
mod graph_validation_config;
mod ipc_mmio;
mod linux_boot_clocks;
mod linux_cache;
mod linux_dma_buffer;
mod linux_pas;
mod linux_power;

use accelerator_device::AcceleratorDevice;

kernel::module_platform_driver! {
    type: AcceleratorDevice<'static>,
    name: "sm8250_npu",
    description: "Qualcomm SM8250 neural processing unit",
    license: "GPL v2",
}
