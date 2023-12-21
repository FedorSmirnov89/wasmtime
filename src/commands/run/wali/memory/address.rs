use anyhow::{bail, Result};
use libc::c_void;

use super::AddressCalculation;

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
    pub(crate) fn new(offset: i32, memory: impl AddressCalculation) -> Self {
        // TODO see whether this can be handled nicer
        // Problem is that this is called directly in the host functions and we
        // don't really have nice error handling there
        if offset < 0 {
            panic!("offset must be non-negative");
        }
        if offset as usize >= memory.memory_size() {
            panic!("offset must be smaller than the memory size");
        }
        Self(offset)
    }

    pub(crate) fn to_host_address(&self, memory: impl AddressCalculation) -> HostAddress {
        let offset = self.0 as usize;
        memory.address_of_offset(offset).into()
    }

    pub(crate) fn offset(&self) -> usize {
        self.0 as usize
    }
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

impl From<*mut c_void> for HostAddress {
    fn from(value: *mut c_void) -> Self {
        Self(value as usize)
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
    pub(crate) fn to_wasm_address(&self, memory: impl AddressCalculation) -> Result<WasmAddress> {
        let base_address = memory.address_of_offset(0);
        let offset = self.0 - base_address;
        if offset >= memory.memory_size() {
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

    pub(crate) fn as_usize(self) -> usize {
        self.0
    }

    pub(crate) fn as_i32_ptr(self) -> *mut i32 {
        self.0 as *mut i32
    }
}
