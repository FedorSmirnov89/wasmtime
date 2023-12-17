//! Module for the code to run WASM module compiled against the WALI interface.
//! (eventually, this should (a) live in a separate crate and (b) be only compiled
//! if the `wali` feature is enabled)

use std::ffi::CString;

use anyhow::{anyhow, Context, Result};

use super::RunCommand;

mod host_functions;
mod memory;
mod run;

///
/// Used as the module store for WALI modules. Maintains the host state per module
/// instance.
///
pub(crate) struct WaliCtx {
    arguments: Vec<String>,
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

        Self { arguments }
    }

    ///
    /// Returns the argument at the provided index as CString (i.e., in the shape in
    /// which it will be writting into the module memory)
    ///
    fn arg_as_c_string(&self, index: usize) -> Result<CString> {
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
    fn arg_len(&self, index: usize) -> Result<usize> {
        let len = self.arg_as_c_string(index)?.as_bytes().len();
        Ok(len)
    }
}
