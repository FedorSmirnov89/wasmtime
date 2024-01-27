//! Module defining how the module store storing the runtime context of a module instance looks like

use std::sync::{Arc, Mutex, MutexGuard};

use anyhow::{anyhow, Result};
use wasmtime::{InstancePre, Linker, Module, SharedMemory};

use crate::commands::RunCommand;

mod arguments;
pub(crate) mod mmap;
pub(crate) mod threads;

pub(crate) use mmap::*;

use self::threads::ThreadCtx;

///
/// Used to retrieve the inner context for situations where we cannot work with results
/// an error is indicated by returning -1
///
#[macro_export]
macro_rules! try_lock_ctx {
    ($ctx: ident) => {
        match $ctx.lock() {
            Ok(ctx_lock) => ctx_lock,
            Err(_) => {
                tracing::error!("failed to lock ctx");
                return -1;
            }
        }
    };
}

///
/// Used as the module store for WALI modules. Maintains the host state per module
/// instance.
///
pub(crate) struct WaliCtx {
    inner: Arc<Mutex<InnerCtx>>,
}

impl Clone for WaliCtx {
    fn clone(&self) -> Self {
        let cloned_inner = Arc::clone(&self.inner);
        Self {
            inner: cloned_inner,
        }
    }
}

impl WaliCtx {
    pub(crate) fn new(run_cmd: &RunCommand) -> Self {
        let inner = InnerCtx::new(run_cmd);
        Self {
            inner: Arc::new(Mutex::new(inner)),
        }
    }

    pub(crate) fn lock(&self) -> Result<MutexGuard<InnerCtx>> {
        self.inner
            .lock()
            .map_err(|_| anyhow!("could not lock inner ctx"))
    }
}

pub(crate) struct InnerCtx {
    arguments: Vec<String>,
    mmap_data: MMapData,
    thread_ctx: ThreadCtx,
    memory: Option<SharedMemory>,
}

impl InnerCtx {
    ///
    /// Builds a wali ctx by reading out the provided arguments from the run command
    ///
    fn new(run_cmd: &RunCommand) -> Self {
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
        let thread_ctx = ThreadCtx::default();
        Self {
            arguments,
            mmap_data: MMapData::default(),
            thread_ctx,
            memory: None,
        }
    }

    pub(crate) fn mmap_data(&mut self) -> &mut MMapData {
        &mut self.mmap_data
    }

    pub(crate) fn instance_pre(&self) -> Result<&InstancePre<WaliCtx>> {
        self.thread_ctx.instance_pre()
    }

    pub(crate) fn set_memory(&mut self, memory: SharedMemory) {
        self.memory = Some(memory);
    }

    pub(crate) fn get_memory(&self) -> Result<&SharedMemory> {
        self.memory.as_ref().ok_or(anyhow!("memory not set"))
    }

    pub(crate) fn precompile_module(
        &mut self,
        module: Module,
        linker: &Linker<WaliCtx>,
    ) -> Result<()> {
        self.thread_ctx.precompile_module(module, linker)
    }

    pub(crate) fn thread_ctx(&mut self) -> &mut ThreadCtx {
        &mut self.thread_ctx
    }
}
