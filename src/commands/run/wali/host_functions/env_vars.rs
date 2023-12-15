//! Module for the host functions providing the environment variables to the module.

pub(super) fn get_init_envfile(faddr: i32, fsize: i32) -> i32 {
    println!("module want to read env file: address '{faddr}'; size: '{fsize}'");
    println!("for now, we always behave as if no env file was provided");
    0
}
