use anyhow::Result;
use wasmtime::Caller;

use crate::commands::run::wali::WaliCtx;

use tracing::warn;

pub(super) fn wasm_thread_spawn(
    caller: Caller<'_, WaliCtx>,
    setup_fn_ptr: i32,
    arg_ptr: i32,
) -> i32 {
    0
    // match wasm_thread_spawn_fallible(caller, setup_fn_ptr, arg_ptr) {
    //     Ok(r) => r,
    //     Err(e) => {
    //         warn!("error when spawning thread: {e}");
    //         -1
    //     }
    // }
}

fn wasm_thread_spawn_fallible(
    caller: Caller<'_, WaliCtx>,
    setup_fn_ptr: i32,
    arg_ptr: i32,
) -> Result<i32> {
    println!("wasm_thread_spawn: fn at {setup_fn_ptr}; arg at {arg_ptr}");

    let ctx = caller.data();

    ctx.spawn_thread()?;

    Ok(0)
}
