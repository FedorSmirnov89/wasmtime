//! Module for the code to run WASM module compiled against the WALI interface.
//! (eventually, this should (a) live in a separate crate and (b) be only compiled
//! if the `wali` feature is enabled)

mod host_functions;
mod memory;
mod run;
mod store;

pub(crate) use store::WaliCtx;

#[cfg(test)]
mod test;
