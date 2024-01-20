use wasmtime::Caller;

use crate::commands::run::wali::WaliCtx;

pub(super) fn wasm_thread_spawn(
    caller: Caller<'_, WaliCtx>,
    setup_fn_ptr: i32,
    arg_ptr: i32,
) -> i32 {
    println!("wasm_thread_spawn: fn at {setup_fn_ptr}; arg at {arg_ptr}");
    0
}
