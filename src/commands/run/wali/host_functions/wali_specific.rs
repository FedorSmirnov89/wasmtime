//! Module for the host functions which are Wali-specific, i.e., are introduced during the compilation to Wasm
//! or used internally by the runtime

pub(crate) fn call_ctors() {
    println!("module has executed the '__call_ctors' host function");
}

pub(crate) fn call_dtors() {
    println!("module has executed the '__call_dtors' host function");
}

pub(crate) fn proc_exit(exit_code: i32) {
    println!("module has executed the 'exit' host function");
    println!("module executed successfully; exiting process");
    std::process::exit(exit_code);
}
