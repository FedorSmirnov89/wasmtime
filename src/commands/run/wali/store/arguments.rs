use std::ffi::CString;

use anyhow::{anyhow, Context, Result};

use super::WaliCtx;

impl WaliCtx {
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
