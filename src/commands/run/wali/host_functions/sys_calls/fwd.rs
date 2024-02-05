//! Module for the generation of the host functions which forward the system calls made
//! within the WASM modules to the host OS (possibly translating the memory pointers in the
//! process).

#![allow(unused_parens)] // for the macro (we have unnecessary parens when generating sys calls with one argument)

///
/// Macro used to generate the use statements required by the functions generated by the `syscall_fwd` macro.
///
macro_rules! syscall_fwd_prelude {
    () => {
        use wasmtime::Caller;

        use crate::commands::run::wali::{memory::address::WasmAddress, WaliCtx};

        use anyhow::Result;

        use tracing::{error, info};
    };
}

///
/// Macro used to generate the host functions which forward the system calls made from within
/// the WASM modules to the host OS (possibly translating the memory pointers in the process).
///
/// Usage:
///
/// `syscall_fwd! {name: "write", num: 1, args: [a1, m2, a3]}`
///
/// will generate a function called `write` which will accept 3 arguments and use them to
/// make the system call number 1 (in this case, `write`), as defined in the `libc` crate.
/// Furthermore, it will treat the second argument (m2) as the offset into the memory of the
/// WASM module (all arguments whose identifier starts with an 'm' are treated as WASM addresses).
/// Arguments representing WASM addresses are translated into host addresses prior to being provided
/// to the system call to the host OS.
///
macro_rules! syscall_fwd {
    // default case -- all arguments are i32
    (name: $name: literal, num: $num: literal, args: [$($arg: ident),+]) => {
        paste::item!{
            pub(crate) fn [<$name>](caller: Caller<'_, WaliCtx>, $($arg: i32),+) -> i64 {
                let tid = unsafe{libc::pthread_self()};
                info!("module has executed the '{}' host function from thread {}.", $name, tid);
                match [<$name _impl>](caller, $($arg),+) {
                    Ok(r) => r,
                    Err(e) => {
                        error!("error when calling '{}': {e}", $name);
                        -1
                    }
                }
            }

            fn [<$name _impl>](caller: Caller<'_, WaliCtx>, $($arg: i32),+) -> Result<i64>{

                let ($($arg),+) = ($(
                    if stringify!($arg).starts_with("m"){
                        let ctx_inner = caller.data().lock()?;
                        let memory = ctx_inner.get_memory()?;
                        WasmAddress::new($arg, memory).to_host_address(memory).into()
                    }else{
                        $arg as libc::c_long
                    }
                ),+);

                let sys_call_result = unsafe {libc::syscall($num, $($arg),+)};
                Ok(sys_call_result)
            }
        }
    };

    // extra case without arguments
    (name: $name: literal, num: $num: literal) => {
        paste::item!{
            pub(crate) fn [<$name>]() -> i64 {
                let tid = unsafe{libc::pthread_self()};
                info!("module has executed the '{}' host function from thread {}.", $name, tid);
                unsafe { libc::syscall($num) }
            }
        }
    };

    // extra case where the types of the arguments are explicitly specified
    (name: $name: literal, num: $num: literal, args: [$($arg: ident: $arg_type: ty),+]) => {
        paste::item!{
            pub(crate) fn [<$name>](caller: Caller<'_, WaliCtx>, $($arg: $arg_type),+) -> i64 {
                let tid = unsafe{libc::pthread_self()};
                info!("module has executed the '{}' host function from thread {}.", $name, tid);
                match [<$name _impl>](caller, $($arg),+) {
                    Ok(r) => r,
                    Err(e) => {
                        error!("error when calling '{}': {e}", $name);
                        -1
                    }
                }
            }

            #[allow(trivial_numeric_casts)]
            fn [<$name _impl>](caller: Caller<'_, WaliCtx>, $($arg: $arg_type),+) -> Result<i64>{

                let ($($arg),+) = ($(
                    if stringify!($arg).starts_with("m"){
                        let ctx_inner = caller.data().lock()?;
                        let memory = ctx_inner.get_memory()?;
                        WasmAddress::new($arg as i32, memory).to_host_address(memory).into()
                    }else{
                        $arg as libc::c_long
                    }
                ),+);

                let sys_call_result = unsafe {libc::syscall($num, $($arg),+)};
                Ok(sys_call_result)
            }
        }
    };
}

syscall_fwd_prelude!();

syscall_fwd! {name: "read", num: 0, args: [a1, m2, a3]}
syscall_fwd! {name: "write", num: 1, args: [a1, m2, a3]}
syscall_fwd! {name: "close", num: 3, args: [a1]}
syscall_fwd! {name: "fstat", num: 5, args: [a1, m2]}
syscall_fwd! {name: "mprotect", num: 10, args: [m1, a2, a3]}
syscall_fwd! {name: "rt_sigprocmask", num: 14, args: [a1, m2, m3, a4]}
syscall_fwd! {name: "ioctl", num: 16, args: [a1, a2, m3]}
syscall_fwd! {name: "nanosleep", num: 35, args: [m1, m2]}
syscall_fwd! {name: "socket", num: 41, args: [a1, a2, a3]}
syscall_fwd! {name: "connect", num: 42, args: [a1, m2, a3]}
syscall_fwd! {name: "accept", num: 43, args: [a1, m2, m3]}
syscall_fwd! {name: "sendto", num: 44, args: [a1, m2, a3, a4, m5, a6]}
syscall_fwd! {name: "shutdown", num: 48, args: [a1, a2]}
syscall_fwd! {name: "bind", num: 49, args: [a1, m2, a3]}
syscall_fwd! {name: "listen", num: 50, args: [a1, a2]}
syscall_fwd! {name: "setsockopt", num: 54, args: [a1, a2, a3, m4, a5]}
syscall_fwd! {name: "kill", num: 62, args: [a1, a2]}
syscall_fwd! {name: "uname", num: 63, args: [m1]}
syscall_fwd! {name: "flock", num: 73, args: [a1, a2]}
syscall_fwd! {name: "get_cwd", num: 79, args: [m1, a2]}
syscall_fwd! {name: "setpgid", num: 109, args: [a1, a2]}
syscall_fwd! {name: "fstatfs", num: 138, args: [a1, m2]}
syscall_fwd! {name: "gettid", num: 186}
syscall_fwd! {name: "futex", num: 202, args: [m1, a2, a3, m4, m5, a6]}
syscall_fwd! {name: "getdents64", num: 217, args: [a1, m2, a3]}
syscall_fwd! {name: "set_tid_address", num: 218, args: [m1]}
syscall_fwd! {name: "clock_gettime", num: 228, args: [a1, m2]}
syscall_fwd! {name: "clock_nanosleep", num: 230, args: [a1, a2, m3, m4]}
syscall_fwd! {name: "utimensat", num: 280, args: [a1, m2, m3, a4]}

syscall_fwd! {name: "lseek", num: 8, args: [a1: i32, a2: i64, a3: i32]}

// Architecture-specific syscalls (currently only for x86_64)
#[cfg(target_arch = "x86_64")]
mod arch_specific_x86 {
    #![allow(unused_parens)] // for the macro (we have unnecessary parens when generating sys calls with one argument)

    syscall_fwd_prelude!();

    syscall_fwd! {name: "open", num: 2, args: [m1, a2, a3]}
    syscall_fwd! {name: "stat", num: 4, args: [m1, m2]}
    syscall_fwd! {name: "lstat", num: 6, args: [m1, m2]}
    syscall_fwd! {name: "access", num: 21, args: [m1, a2]}
    syscall_fwd! {name: "pipe", num: 22, args: [m1]}
    syscall_fwd! {name: "dup", num: 32, args: [a1]}
    syscall_fwd! {name: "dup2", num: 33, args: [a1, a2]}
    syscall_fwd! {name: "alarm", num: 37, args: [a1]}
    syscall_fwd! {name: "fork", num: 57}
    syscall_fwd! {name: "fcntl", num: 72, args: [a1, a2, a3]}
    syscall_fwd! {name: "dup3", num: 292, args: [a1, a2, a3]}
}

pub(crate) use arch_specific_x86::*;
