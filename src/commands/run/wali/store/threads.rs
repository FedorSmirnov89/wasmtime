use std::sync::Arc;

use tracing::warn;

use anyhow::Result;
use wasmtime::{InstancePre, Linker, Module};

use super::WaliCtx;

pub(crate) struct ThreadCtx {
    instance_pre: Arc<InstancePre<WaliCtx>>,
}

impl ThreadCtx {
    pub(crate) fn new(module: &Module, linker: &Linker<WaliCtx>) -> Result<Self> {
        warn!("creating thread context");
        let instance_pre = linker.instantiate_pre(module)?;
        warn!("precompiled instance");
        Ok(Self {
            instance_pre: Arc::new(instance_pre),
        })
    }

    pub(crate) fn spawn(&self) -> Result<()> {
        warn!("spawning thread from the thread context");
        Ok(())
    }
}
