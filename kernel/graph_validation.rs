// SPDX-License-Identifier: GPL-2.0-only

use kernel::{
    alloc::flags::GFP_KERNEL,
    device::{Bound, Device, property::FwNode},
    error::{
        Result,
        code::{EBADMSG, EINVAL, ENODATA, ENODEV},
    },
    prelude::*,
    str::CString,
};

use crate::{
    GraphRelocations,
    firmware_blob::FirmwareBlob,
    graph_exchange::GraphExchange,
    graph_validation_config::GraphValidationConfig,
    ipc::graph::{Execute, ExecutionBuffer, Load},
    ipc_mmio::IpcMmio,
    linux_dma_buffer::LinuxDmaBuffer,
};

pub(crate) struct GraphValidation<'a> {
    config: GraphValidationConfig,
    graph: LinuxDmaBuffer<'a>,
    _workspace: LinuxDmaBuffer<'a>,
    input: LinuxDmaBuffer<'a>,
    output: LinuxDmaBuffer<'a>,
    expected_output: Option<FirmwareBlob>,
}

impl<'a> GraphValidation<'a> {
    const OUTPUT_MARKER: u8 = 0xa5;

    pub(crate) fn prepare(dev: &'a Device<Bound>) -> Result<Option<Self>> {
        let node = dev.fwnode().ok_or(ENODEV)?;
        if !node.property_present(c"qcom,validation-aco") {
            return Ok(None);
        }
        let config = GraphValidationConfig::read(dev, &node)?;
        let aco = Self::asset(dev, &node, c"qcom,validation-aco", config.graph_size)?;
        let weights = Self::asset(dev, &node, c"qcom,validation-weights", config.weights_size)?;
        let relocations = Self::asset(
            dev,
            &node,
            c"qcom,validation-relocations",
            config.relocation_bytes,
        )?;
        let tensor = Self::asset(dev, &node, c"qcom,validation-input", config.input_size)?;

        let expected_output = node
            .property_present(c"qcom,validation-output")
            .then(|| Self::asset(dev, &node, c"qcom,validation-output", config.output_size))
            .transpose()?;

        let mut graph = LinuxDmaBuffer::allocate(dev, config.graph_size)?;
        let mut workspace = LinuxDmaBuffer::allocate(dev, config.workspace_size)?;
        let mut input = LinuxDmaBuffer::allocate(dev, config.input_size)?;
        let output = LinuxDmaBuffer::allocate(dev, config.output_size)?;
        let mut patched = KVVec::new();
        patched.extend_from_slice(aco.data()?, GFP_KERNEL)?;
        GraphRelocations::apply(
            &mut patched,
            relocations.data()?,
            workspace.iova(),
            config.workspace_size,
        )
        .map_err(|_| EINVAL)?;

        // SAFETY: The firmware has not started and has not received any DMA addresses.
        unsafe {
            graph.write(0, &patched)?;
            workspace.write(0, weights.data()?)?;
            input.write(0, tensor.data()?)?;
        }

        Ok(Some(Self {
            config,
            graph,
            _workspace: workspace,
            input,
            output,
            expected_output,
        }))
    }

    pub(crate) fn run(&mut self, dev: &Device, ipc: &mut IpcMmio<'_>) -> Result {
        let mut exchange = GraphExchange::new(ipc)?;
        let handle = exchange.load(&Load {
            network_id: self.config.network_id,
            first_block_size_bytes: self.config.first_block_size,
            graph_iova: self.graph.iova(),
            layer_count: self.config.layer_count,
            patches: &self.config.load_patches,
        })?;
        let first = self.execute(&mut exchange, handle)?;
        let second = self.execute(&mut exchange, handle)?;
        exchange.unload(handle)?;
        if first != second || first.iter().all(|byte| *byte == Self::OUTPUT_MARKER) {
            return Err(ENODATA);
        }

        if let Some(expected) = &self.expected_output {
            if expected.data()? != first.as_slice() {
                dev_err!(dev, "NPU validation output differs from reference\n");
                return Err(EBADMSG);
            }
            dev_info!(dev, "NPU validation output matched the reference tensor\n");
        }
        dev_info!(
            dev,
            "NPU validation graph completed twice with identical output\n"
        );
        Ok(())
    }

    fn execute(&mut self, exchange: &mut GraphExchange<'_, '_>, handle: u32) -> Result<KVVec<u8>> {
        let mut output = KVVec::new();
        output.resize(self.config.output_size, Self::OUTPUT_MARKER, GFP_KERNEL)?;
        // SAFETY: The output has not been submitted for this execution, and prior executions finished.
        unsafe { self.output.write(0, &output)? };
        exchange.execute(&Execute {
            handle,
            buffers: &[
                ExecutionBuffer {
                    iova: self.input.iova(),
                    patch_id: self.config.input_patch_id,
                },
                ExecutionBuffer {
                    iova: self.output.iova(),
                    patch_id: self.config.output_patch_id,
                },
            ],
        })?;
        // SAFETY: The matching successful completion transfers the output back to the CPU.
        unsafe { self.output.read(0, &mut output)? };
        Ok(output)
    }

    fn asset(dev: &Device, node: &FwNode, property: &CStr, bytes: usize) -> Result<FirmwareBlob> {
        let path = node.property_read::<CString>(property).required_by(dev)?;
        let blob = FirmwareBlob::request(dev, &path)?;
        if blob.data()?.len() != bytes {
            return Err(EINVAL);
        }
        Ok(blob)
    }
}
