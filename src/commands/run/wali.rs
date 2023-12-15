//! Module for the code to run WASM module compiled against the WALI interface.
//! (eventually, this should (a) live in a separate crate and (b) be only compiled
//! if the `wali` feature is enabled)

use anyhow::Result;
use wasmtime::Linker;

use super::RunCommand;

mod run;

///
/// Used as the module store for WALI modules. Maintains the host state per module
/// instance.
///
#[derive(Default)]
pub(crate) struct WaliCtx {}

impl RunCommand {
    pub(super) fn link_wali_host_functions(&self, linker: &mut Linker<WaliCtx>) -> Result<()> {
        println!("linking wali functions");
        Ok(())
    }
}
