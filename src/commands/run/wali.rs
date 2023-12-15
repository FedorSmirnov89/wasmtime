//! Module for the code to run WASM module compiled against the WALI interface.
//! (eventually, this should (a) live in a separate crate and (b) be only compiled
//! if the `wali` feature is enabled)

mod host_functions;
mod memory;
mod run;

///
/// Used as the module store for WALI modules. Maintains the host state per module
/// instance.
///
#[derive(Default)]
pub(crate) struct WaliCtx {}
