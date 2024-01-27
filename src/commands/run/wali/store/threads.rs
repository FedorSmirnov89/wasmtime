use std::panic::{catch_unwind, AssertUnwindSafe};

use anyhow::{anyhow, Result};
use wasmtime::{InstancePre, Linker, Module, Store};

use super::WaliCtx;

const FUNC_NAME_MODULE_FUNC: &str = "__wasm_thread_start_libc";

#[derive(Default)]
pub(crate) struct ThreadCtx {
    instance_pre: Option<InstancePre<WaliCtx>>,
    thread_count: u32,
}

impl ThreadCtx {
    pub(crate) fn precompile_module(
        &mut self,
        module: Module,
        linker: &Linker<WaliCtx>,
    ) -> Result<()> {
        self.instance_pre = Some(linker.instantiate_pre(&module)?);
        Ok(())
    }

    pub(crate) fn instance_pre(&self) -> Result<&InstancePre<WaliCtx>> {
        self.instance_pre
            .as_ref()
            .ok_or(anyhow!("instance_pre not set"))
    }

    ///
    /// Will return the ID of the newly spawned thread
    ///
    pub(crate) fn spawn(&mut self, ctx: WaliCtx, _func_idx: i32, arg_ptr: i32) -> Result<i32> {
        let cur_thread_count = self.thread_count;
        self.thread_count += 1;
        let thread_builder =
            std::thread::Builder::new().name(format!("wali-thread-{cur_thread_count}"));
        let ctx_clone = ctx.clone();

        let (tid_sender, tid_recv) = std::sync::mpsc::channel();

        thread_builder.spawn(move || {
            // query the ID of the spqwned thread and send it back to the main thread
            let tid = unsafe { libc::pthread_self() };
            tid_sender
                .send(tid)
                .expect("failed sending thread id of new thread back");

            // create a new module instance and run the appropriate function
            let engine = {
                // release lock after getting engine
                let ctx_inner = ctx_clone.lock().expect("could not lock ctx");
                ctx_inner
                    .instance_pre()
                    .expect("instance_pre not set")
                    .module()
                    .engine()
                    .clone()
            };

            tracing::debug!("THREAD INSTANCE: engine retrieved");

            let mut store = Store::new(&engine, ctx_clone.clone());
            let instance = {
                // release the lock after instantiating a new module instance
                let ctx_inner = ctx.lock().expect("failed to get ctx lock");
                ctx_inner
                    .instance_pre()
                    .expect("instance pre not set")
                    .instantiate(&mut store)
                    .expect("faild to instantiate instance in thread")
            };
            tracing::debug!("THREAD INSTANCE: new module instantiated");

            let thread_entry_point = instance
                .get_typed_func::<(i32, i32), ()>(&mut store, FUNC_NAME_MODULE_FUNC)
                .expect("failed to get thread entry point function from module");

            tracing::debug!("THREAD INSTANCE: entry point retrieved");

            let result = catch_unwind(AssertUnwindSafe(|| {
                match thread_entry_point.call(&mut store, (tid as i32, arg_ptr)) {
                    Ok(_) => tracing::info!("wasi thread {cur_thread_count} exited normally"),
                    Err(e) => {
                        tracing::error!(
                            "exiting wasi thread {cur_thread_count} due to error: {e:?}"
                        );
                    }
                }
            }));

            match result {
                Ok(_) => tracing::debug!("thread entry point function terminated normally"),
                Err(e) => tracing::error!("thread entry point function paniced: {e:?}"),
            }
        })?;

        let tid = tid_recv.recv()?;

        tracing::info!("Received thread id: {tid} from spawned thread");
        Ok(tid as i32)
    }
}
