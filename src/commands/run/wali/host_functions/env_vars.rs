//! Module for the host functions providing the environment variables to the module.

use tracing::info;

pub(super) fn get_init_envfile(faddr: i32, fsize: i32) -> i32 {
    info!("module wants to read env file: address '{faddr}'; size: '{fsize}'");
    info!("for now, we always behave as if no env file was provided");
    0
}
