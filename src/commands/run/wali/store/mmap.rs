use anyhow::{bail, Result};
use wasmtime_environ::WASM_PAGE_SIZE;

use crate::commands::run::wali::memory::PageAlignment;

///
/// Used to store the data relevant for the mmap syscall. Provided within a mutex guard
/// to synchronize between threads performing mmap syscalls.
///
pub(crate) struct MMapData {
    pub(crate) n_mmap_pages: usize,
    pub(crate) page_size_native: usize,
    pub(crate) page_size_wasm: usize,
    base_size: Option<usize>,
}

impl Default for MMapData {
    fn default() -> Self {
        let page_size_wasm = WASM_PAGE_SIZE as usize;
        let page_size_native = unsafe { libc::sysconf(libc::_SC_PAGESIZE) } as usize;

        Self {
            n_mmap_pages: 0,
            page_size_wasm,
            page_size_native,
            base_size: None,
        }
    }
}

impl MMapData {
    pub(crate) fn init_base_size(&mut self, memsize: usize) {
        if self.base_size.is_none() {
            self.base_size = Some(memsize);
        }
    }

    pub(crate) fn base_size(&self) -> Result<usize> {
        if let Some(base_size) = self.base_size {
            Ok(base_size)
        } else {
            bail!("base size not initialized")
        }
    }

    pub(crate) fn add_mmap_pages(&mut self, n_pages: usize) {
        self.n_mmap_pages += n_pages;
    }

    pub(crate) fn unmap_pages_from_end(&mut self, n_pages: usize) {
        self.n_mmap_pages -= n_pages;
    }

    pub(crate) fn memory_end_aligned(&self, base_address: usize) -> Result<usize> {
        let page_size_native = self.page_size_native;

        let base_memory_size = self.base_size()?;
        let mmap_memory_size = self.n_mmap_pages * page_size_native;

        let address_unaligned = base_address + base_memory_size + mmap_memory_size;
        Ok(address_unaligned.page_aligned(page_size_native))
    }
}
