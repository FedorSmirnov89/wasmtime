//! Module containing the functionality which replaces the code normally run
//! by wasmtime in the case where we run a WALI module.

use anyhow::{anyhow, Context, Result};
use wasmtime::{Linker, Store};

use crate::{commands::RunCommand, common::RunTarget};

use super::WaliCtx;

impl RunCommand {
    ///
    /// Ugly hack to use a different function for loading the module, since,
    /// when using WALI, we use a different store and don't have things like
    /// the epoch handler or preloaded modules. Will do a more elegant solution
    /// when the features set of WALI modules becomes a bit more clear.
    ///
    pub(crate) fn load_wali_module(
        &self,
        store: &mut Store<WaliCtx>,
        linker: &mut Linker<WaliCtx>,
        module: &RunTarget,
    ) -> Result<()> {
        let result = {
            let module = module.unwrap_core();
            let instance = linker.instantiate(&mut *store, &module).context(format!(
                "failed to instantiate {:?}",
                self.module_and_args[0]
            ))?;

            // If `_initialize` is present, meaning a reactor, then invoke
            // the function.
            if let Some(func) = instance.get_func(&mut *store, "_initialize") {
                func.typed::<(), ()>(&store)?.call(&mut *store, ())?;
            }

            // Look for the specific function provided or otherwise look for
            // "" or "_start" exports to run as a "main" function.
            let func = if let Some(name) = &self.invoke {
                Some(
                    instance
                        .get_func(&mut *store, name)
                        .ok_or_else(|| anyhow!("no func export named `{}` found", name))?,
                )
            } else {
                instance
                    .get_func(&mut *store, "")
                    .or_else(|| instance.get_func(&mut *store, "_start"))
            };

            match func {
                Some(func) => self.invoke_func(store, func),
                None => Ok(()),
            }
        };
        result
    }
}
