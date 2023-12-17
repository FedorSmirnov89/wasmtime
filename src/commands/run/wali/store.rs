//! Module defining how the module store storing the runtime context of a module instance looks like

use std::{
    ffi::CString,
    sync::{Mutex, MutexGuard},
};

use anyhow::{anyhow, bail, Context, Result};
use wasmtime_environ::WASM_PAGE_SIZE;

use crate::commands::RunCommand;

use super::memory::PageAlignment;

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

///
/// Used as the module store for WALI modules. Maintains the host state per module
/// instance.
///
pub(crate) struct WaliCtx {
    arguments: Vec<String>,
    mmap_data: Mutex<MMapData>,
}

impl WaliCtx {
    ///
    /// Builds a wali ctx by reading out the provided arguments from the run command
    ///
    pub(crate) fn new(run_cmd: &RunCommand) -> Self {
        let arg_iterator = run_cmd.module_and_args.iter();
        let arguments = arg_iterator
            .skip(1) // first argument is the command name
            .map(|os_arg| {
                os_arg
                    .to_str()
                    .expect("could not convert arg to utf-8")
                    .into()
            })
            .collect();

        Self {
            arguments,
            mmap_data: Mutex::new(MMapData::default()),
        }
    }

    pub(crate) fn lock_mmap_data(&self) -> Result<MutexGuard<MMapData>> {
        self.mmap_data
            .lock()
            .map_err(|_| anyhow!("could not lock mmap data"))
    }

    ///
    /// Returns the argument at the provided index as CString (i.e., in the shape in
    /// which it will be writting into the module memory)
    ///
    pub(crate) fn arg_as_c_string(&self, index: usize) -> Result<CString> {
        let arg = self
            .arguments
            .get(index)
            .ok_or_else(|| anyhow!("argument index out of bounds"))?;
        let c_string = CString::new(arg.as_str()).context("converting arg string to C string")?;
        Ok(c_string)
    }

    ///
    /// Returns the length of the argument at the provided index (the length of the byte array
    /// it will occupy in the WASM memory, i.e., the length of the C string representation of
    /// the argument)
    ///
    pub(crate) fn arg_byte_len(&self, index: usize) -> Result<usize> {
        let len = self.arg_as_c_string(index)?.as_bytes().len();
        Ok(len)
    }

    ///
    /// Returns the number of arguments provided to the module
    ///
    pub(crate) fn arg_len(&self) -> usize {
        self.arguments.len()
    }
}
