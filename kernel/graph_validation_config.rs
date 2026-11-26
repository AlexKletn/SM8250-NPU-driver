// SPDX-License-Identifier: GPL-2.0-only

use kernel::{
    alloc::flags::GFP_KERNEL,
    device::{Device, property::FwNode},
    error::{Result, code::EINVAL},
    prelude::*,
};

use crate::{
    GraphRelocations,
    ipc::{Queue, graph::LoadPatch},
};

pub(crate) struct GraphValidationConfig {
    pub(crate) graph_size: usize,
    pub(crate) workspace_size: usize,
    pub(crate) weights_size: usize,
    pub(crate) relocation_bytes: usize,
    pub(crate) input_size: usize,
    pub(crate) output_size: usize,
    pub(crate) network_id: u32,
    pub(crate) first_block_size: u32,
    pub(crate) layer_count: u32,
    pub(crate) input_patch_id: u32,
    pub(crate) output_patch_id: u32,
    pub(crate) load_patches: KVec<LoadPatch>,
}

impl GraphValidationConfig {
    pub(crate) fn read(dev: &Device, node: &FwNode) -> Result<Self> {
        let read = |name| node.property_read::<u32>(name).required_by(dev);
        let relocation_count = read(c"qcom,validation-relocation-count")? as usize;
        let config = Self {
            graph_size: read(c"qcom,validation-aco-size")? as usize,
            workspace_size: read(c"qcom,validation-workspace-size")? as usize,
            weights_size: read(c"qcom,validation-weights-size")? as usize,
            relocation_bytes: relocation_count
                .checked_mul(GraphRelocations::RECORD_BYTES)
                .ok_or(EINVAL)?,
            input_size: read(c"qcom,validation-input-size")? as usize,
            output_size: read(c"qcom,validation-output-size")? as usize,
            network_id: read(c"qcom,validation-network-id")?,
            first_block_size: read(c"qcom,validation-first-block-size")?,
            layer_count: read(c"qcom,validation-layer-count")?,
            input_patch_id: read(c"qcom,validation-input-patch-id")?,
            output_patch_id: read(c"qcom,validation-output-patch-id")?,
            load_patches: Self::read_patches(dev, node)?,
        };
        if [
            config.graph_size,
            config.workspace_size,
            config.weights_size,
            config.input_size,
            config.output_size,
            config.first_block_size as usize,
        ]
        .contains(&0)
            || config.weights_size > config.workspace_size
            || config.first_block_size as usize > config.graph_size
        {
            return Err(EINVAL);
        }
        Ok(config)
    }

    fn read_patches(dev: &Device, node: &FwNode) -> Result<KVec<LoadPatch>> {
        const WORDS_PER_PATCH: usize = 6;
        const LOAD_HEADER_WORDS: usize = 9;
        const RESERVED_QUEUE_WORDS: usize = 1;
        let property = c"qcom,validation-load-patches";
        if !node.property_present(property) {
            return Ok(KVec::new());
        }

        let count = node.property_count_elem::<u32>(property)?;
        let max_words = Queue::AppsCommand.capacity_bytes() / size_of::<u32>()
            - LOAD_HEADER_WORDS
            - RESERVED_QUEUE_WORDS;
        if !count.is_multiple_of(WORDS_PER_PATCH) || count > max_words {
            return Err(EINVAL);
        }
        let words = node
            .property_read_array_vec::<u32>(property, count)?
            .required_by(dev)?;
        let mut patches = KVec::with_capacity(count / WORDS_PER_PATCH, GFP_KERNEL)?;
        for chunk in words.chunks_exact(WORDS_PER_PATCH) {
            let patch = chunk.try_into().map_err(|_| EINVAL)?;
            patches.push(LoadPatch::from_firmware_words(patch), GFP_KERNEL)?;
        }
        Ok(patches)
    }
}
