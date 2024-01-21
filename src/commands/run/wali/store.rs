//! Module defining how the module store storing the runtime context of a module instance looks like

use std::sync::{Mutex, MutexGuard};

use anyhow::{anyhow, Result};
use wasmtime::{Linker, Module};

use crate::commands::RunCommand;

mod arguments;
pub(crate) mod mmap;
mod threads;

pub(crate) use mmap::*;

use self::threads::ThreadCtx;

///
/// Used as the module store for WALI modules. Maintains the host state per module
/// instance.
///
pub(crate) struct WaliCtx {
    arguments: Vec<String>,
    mmap_data: Mutex<MMapData>,
    thread_ctx: ThreadCtx,
}

impl WaliCtx {
    ///
    /// Builds a wali ctx by reading out the provided arguments from the run command
    ///
    pub(crate) fn new(
        run_cmd: &RunCommand,
        module: &Module,
        linker: &Linker<WaliCtx>,
    ) -> Result<Self> {
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
        let thread_ctx = ThreadCtx::new(module, linker)?;
        let ctx = Self {
            arguments,
            mmap_data: Mutex::new(MMapData::default()),
            thread_ctx,
        };
        Ok(ctx)
    }

    pub(crate) fn lock_mmap_data(&self) -> Result<MutexGuard<MMapData>> {
        self.mmap_data
            .lock()
            .map_err(|_| anyhow!("could not lock mmap data"))
    }

    pub(crate) fn spawn_thread(&self) -> Result<()> {
        self.thread_ctx.spawn()
    }
}
