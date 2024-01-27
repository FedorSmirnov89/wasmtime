//! Module for the code to run WASM module compiled against the WALI interface.
//! (eventually, this should (a) live in a separate crate and (b) be only compiled
//! if the `wali` feature is enabled)

mod host_functions;
mod memory;
mod store;

use anyhow::{anyhow, bail, Context, Result};
pub(crate) use store::WaliCtx;
use wasmtime::{Caller, Engine, Linker, Module, SharedMemory, Store};

use crate::common::RunTarget;

use super::RunCommand;

#[cfg(test)]
mod test;

impl RunCommand {
    ///
    /// Function instantiates the infrastructure which is used by all instances of the current module
    /// (i.e., the main instance started through '_start' and any other instances started through thread
    /// calls), before calling the function to start the main instance.
    ///
    pub(super) fn instantiate_and_run_wali(&self, engine: Engine, main: RunTarget) -> Result<()> {
        let mut linker = wasmtime::Linker::new(&engine);
        self.link_wali_host_functions(&mut linker)
            .context("linking host functions")?;

        let module = match main {
            RunTarget::Core(m) => m,
            RunTarget::Component(_) => bail!("WALI does not support component modules"),
        };
        let wali_ctx = WaliCtx::new(&self);
        let store = Store::new(&engine, wali_ctx.clone());

        let memory = make_shared_memory(&module, &mut linker, &store)?;
        {
            let mut ctx_inner = wali_ctx.lock()?;
            ctx_inner.set_memory(memory);
        }

        add_thread_host_function_to_linker(&mut linker).context("adding thread host function")?;
        linker.define_unknown_imports_as_traps(&module)?;
        {
            // making sure to release the ctx lock
            let mut ctx_inner = wali_ctx.lock()?;
            ctx_inner
                .precompile_module(module, &linker)
                .context("precompiling module")?;
        }
        self.load_main_instance(engine, wali_ctx)
    }

    ///
    /// Function instantiating the instance-specific infra and triggering the execution of the start function
    /// of the main module instance.
    ///
    fn load_main_instance(&self, engine: Engine, wali_ctx: WaliCtx) -> Result<()> {
        let mut store = Store::new(&engine, wali_ctx.clone());

        let instance = {
            // making sure to release the lock after we have the instance
            let inner_ctx = wali_ctx.lock()?;
            let instance_pre = inner_ctx.instance_pre()?;
            instance_pre.instantiate(&mut store)?
        };
        let func = instance
            .get_func(&mut store, "_start")
            .ok_or(anyhow!("module did not export a '_start' function"))?;
        self.invoke_func(&mut store, func)
    }
}

pub(crate) fn add_thread_host_function_to_linker(linker: &mut Linker<WaliCtx>) -> Result<()> {
    tracing::info!("adding thread host function");
    linker.func_wrap(
        "wali",
        "__wasm_thread_spawn",
        move |caller: Caller<'_, WaliCtx>, first_arg: i32, second_arg: i32| -> i32 {
            let ctx = caller.data();
            let Ok(mut ctx_lock) = ctx.lock() else {
                tracing::error!("failed to lock ctx");
                return -1;
            };
            let thread_ctx = ctx_lock.thread_ctx();
            match thread_ctx.spawn(ctx.clone(), first_arg, second_arg) {
                Ok(_thread_id) => 0,
                Err(e) => {
                    tracing::error!("failed to spawn thread: {e}");
                    -1
                }
            }
        },
    )?;
    Ok(())
}

fn make_shared_memory(
    module: &Module,
    linker: &mut Linker<WaliCtx>,
    store: &Store<WaliCtx>,
) -> Result<SharedMemory> {
    for import in module.imports() {
        if let Some(m) = import.ty().memory() {
            if m.is_shared() {
                let mem = SharedMemory::new(module.engine(), m.clone())?;
                linker.define(store, "env", "memory", mem.clone())?;
                return Ok(mem);
            }
        }
    }
    bail!("module does not export a shared memory")
}
