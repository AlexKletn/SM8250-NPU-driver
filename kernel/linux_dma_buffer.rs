// SPDX-License-Identifier: GPL-2.0-only

use core::{
    ffi::{c_int, c_void},
    ptr::{self, NonNull},
};
use kernel::{
    bindings,
    device::{Bound, Device},
    error::{
        Result,
        code::{EINVAL, ENOMEM, ERANGE},
        to_result,
    },
    page::PAGE_SIZE,
};

unsafe extern "C" {
    fn sm8250_npu_dma_init(dev: *mut c_void) -> c_int;
    fn sm8250_npu_dma_iova(table: *mut c_void, length: usize, iova: *mut u32) -> c_int;
    fn sm8250_npu_dma_cpu_begin(
        dev: *mut c_void,
        table: *mut c_void,
        mapping: *mut c_void,
        length: usize,
    );
    fn sm8250_npu_dma_cpu_end(
        dev: *mut c_void,
        table: *mut c_void,
        mapping: *mut c_void,
        length: usize,
        modified: bool,
    );
}

pub(crate) struct LinuxDmaBuffer<'a> {
    dev: &'a Device<Bound>,
    table: NonNull<bindings::sg_table>,
    length: usize,
    mapped_length: usize,
    iova: u32,
}

impl<'a> LinuxDmaBuffer<'a> {
    const DIRECTION: bindings::dma_data_direction = bindings::dma_data_direction::DMA_BIDIRECTIONAL;

    pub(crate) fn initialize_device(dev: &Device<Bound>) -> Result {
        // SAFETY: The bound device is valid for the DMA API call.
        to_result(unsafe { sm8250_npu_dma_init(Self::raw_device(dev).cast()) })
    }

    pub(crate) fn allocate(dev: &'a Device<Bound>, length: usize) -> Result<Self> {
        if length == 0 || u32::try_from(length).is_err() {
            return Err(EINVAL);
        }
        let mapped_length = length.checked_next_multiple_of(PAGE_SIZE).ok_or(EINVAL)?;
        // SAFETY: The bound device outlives the allocation and mapped_length is page-aligned.
        let table = unsafe {
            bindings::dma_alloc_noncontiguous(
                Self::raw_device(dev),
                mapped_length,
                Self::DIRECTION,
                bindings::GFP_KERNEL,
                0,
            )
        };
        let table = NonNull::new(table).ok_or(ENOMEM)?;
        let mut buffer = Self {
            dev,
            table,
            length,
            mapped_length,
            iova: 0,
        };
        // SAFETY: buffer owns the live scatterlist and iova points to writable storage.
        to_result(unsafe {
            sm8250_npu_dma_iova(table.as_ptr().cast(), mapped_length, &mut buffer.iova)
        })?;
        // SAFETY: A newly allocated buffer has not been submitted to the accelerator.
        unsafe {
            buffer.with_cpu_mapping(true, |mapping| {
                // SAFETY: The mapping covers mapped_length bytes.
                mapping.write_bytes(0, mapped_length);
            })?;
        }
        Ok(buffer)
    }

    pub(crate) fn iova(&self) -> u32 {
        self.iova
    }

    /// The caller must ensure that the device has finished accessing this buffer.
    pub(crate) unsafe fn read(&mut self, offset: usize, output: &mut [u8]) -> Result {
        self.check_range(offset, output.len())?;
        // SAFETY: The caller ensures DMA is idle and the validated range fits the mapping.
        unsafe {
            self.with_cpu_mapping(false, |mapping| {
                ptr::copy_nonoverlapping(mapping.add(offset), output.as_mut_ptr(), output.len());
            })
        }
    }

    /// The caller must ensure that the device has finished accessing this buffer.
    pub(crate) unsafe fn write(&mut self, offset: usize, input: &[u8]) -> Result {
        self.check_range(offset, input.len())?;
        // SAFETY: The caller ensures DMA is idle and the validated range fits the mapping.
        unsafe {
            self.with_cpu_mapping(true, |mapping| {
                ptr::copy_nonoverlapping(input.as_ptr(), mapping.add(offset), input.len());
            })
        }
    }

    fn check_range(&self, offset: usize, length: usize) -> Result {
        if offset > self.length || length > self.length - offset {
            return Err(ERANGE);
        }
        Ok(())
    }

    unsafe fn with_cpu_mapping(&mut self, modified: bool, access: impl FnOnce(*mut u8)) -> Result {
        let dev = Self::raw_device(self.dev);
        let table = self.table.as_ptr();
        // SAFETY: self owns the matching device, allocation size and scatterlist.
        let raw = unsafe { bindings::dma_vmap_noncontiguous(dev, self.mapped_length, table) };
        let mapping = NonNull::new(raw).ok_or(ENOMEM)?;
        // SAFETY: The mapping is live and the caller ensures the accelerator is idle.
        unsafe {
            sm8250_npu_dma_cpu_begin(dev.cast(), table.cast(), raw, self.mapped_length);
            access(mapping.as_ptr().cast());
            sm8250_npu_dma_cpu_end(dev.cast(), table.cast(), raw, self.mapped_length, modified);
            bindings::dma_vunmap_noncontiguous(dev, raw);
        }
        Ok(())
    }

    fn raw_device(dev: &Device<Bound>) -> *mut bindings::device {
        ptr::from_ref(dev).cast_mut().cast()
    }
}

impl Drop for LinuxDmaBuffer<'_> {
    fn drop(&mut self) {
        // SAFETY: self owns the live allocation and the device outlives it. Callers retain
        // the buffers until firmware shutdown if a submitted operation fails or times out.
        unsafe {
            bindings::dma_free_noncontiguous(
                Self::raw_device(self.dev),
                self.mapped_length,
                self.table.as_ptr(),
                Self::DIRECTION,
            );
        }
    }
}
