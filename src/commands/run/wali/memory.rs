//! Module for the structs and functions used for translating/reading/writing between the module and the host memory

use std::sync::atomic::AtomicU8;

use wasmtime::SharedMemory;

pub(crate) mod address;
pub(crate) mod writing;

pub(crate) trait AddressCalculation {
    fn address_of_offset(&self, offset: usize) -> usize;
    fn memory_size(&self) -> usize;
}

impl AddressCalculation for &SharedMemory {
    fn address_of_offset(&self, offset: usize) -> usize {
        self.data().get(offset).expect("memory bound violation") as *const _ as usize
    }

    fn memory_size(&self) -> usize {
        self.data().len()
    }
}

// pub(crate) trait AsMemory {
//     fn as_memory(&mut self) -> SharedMemory;
// }

// impl AsMemory for Caller<'_, WaliCtx> {
//     fn as_memory(&mut self) -> SharedMemory {
//         let tid = unsafe { libc::syscall(libc::SYS_gettid) };
//         tracing::debug!("getting memory from thread '{tid}'");
//         let inner_ctx = self.data().lock().expect("could not lock ctx");
//         inner_ctx.get_memory().expect("memory not set")
//     }
// }

pub(crate) trait AsMemorySlice {
    fn as_memory_slice(&self) -> &mut [AtomicU8];
}

impl AsMemorySlice for &SharedMemory {
    fn as_memory_slice(&self) -> &mut [AtomicU8] {
        unsafe {
            std::slice::from_raw_parts_mut(self.data().as_ptr() as *mut AtomicU8, self.data().len())
        }
    }
}

pub(crate) trait PageAlignment {
    fn page_aligned(&self, page_size: usize) -> usize;
}

impl PageAlignment for usize {
    fn page_aligned(&self, page_size: usize) -> usize {
        let offset = *self & (page_size - 1);
        if offset == 0 {
            *self
        } else {
            *self + page_size - offset
        }
    }
}
