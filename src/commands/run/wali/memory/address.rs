use anyhow::{bail, Result};
use libc::c_void;
use wasmtime::SharedMemory;

use super::AsMemory;

///
/// Represents an address in the module memory, specified as an offset from the beginning of the memory.
///
pub(crate) struct WasmAddress(i32);

impl From<WasmAddress> for i32 {
    fn from(value: WasmAddress) -> Self {
        value.0
    }
}

impl WasmAddress {
    pub(crate) fn new(offset: i32, mut memory: impl AsMemory) -> Self {
        // TODO see whether this can be handled nicer
        // Problem is that this is called directly in the host functions and we
        // don't really have nice error handling there
        let memory = memory.as_memory();
        if offset < 0 {
            panic!("offset must be non-negative");
        }
        if offset as usize >= memory.data().len() {
            panic!("offset must be smaller than the memory size");
        }
        Self(offset)
    }

    pub(crate) fn to_host_address(&self, mut memory: impl AsMemory) -> HostAddress {
        let memory = memory.as_memory();
        let offset = self.0 as usize;
        address_of_offset(&memory, offset).into()
    }

    pub(crate) fn offset(&self) -> usize {
        self.0 as usize
    }
}

fn address_of_offset(memory: &SharedMemory, offset: usize) -> usize {
    memory
        .data()
        .get(offset)
        .expect("memory size checked on creation") as *const _ as usize
}

///
/// Represents an address in the host memory.
///
pub(crate) struct HostAddress(usize);

impl From<usize> for HostAddress {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<HostAddress> for usize {
    fn from(value: HostAddress) -> Self {
        value.0
    }
}

impl From<HostAddress> for i64 {
    fn from(value: HostAddress) -> Self {
        value.0 as i64
    }
}

impl HostAddress {
    pub(crate) fn to_wasm_address(&self, mut memory: impl AsMemory) -> Result<WasmAddress> {
        let memory = memory.as_memory();
        let base_address = address_of_offset(&memory, 0);
        let offset = self.0 - base_address;
        if offset >= memory.data().len() {
            bail!("offset must be smaller than the memory size");
        }
        Ok(WasmAddress(offset as i32))
    }

    pub(crate) fn as_i64_ptr(self) -> *mut i64 {
        self.0 as *mut i64
    }

    pub(crate) fn as_void_ptr(self) -> *mut c_void {
        self.0 as *mut c_void
    }
}
