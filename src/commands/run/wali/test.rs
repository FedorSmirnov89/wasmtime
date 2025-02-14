use std::process::{Command, ExitStatus, Stdio};

const PATH_PREFIX: &'static str = include_str!("../local/path_prefix.txt");

fn wali_test(test_name: &'static str) {
    let exit_status = run_wali_module(test_name);
    assert!(exit_status.success());
}

fn run_wali_module(module_name: &'static str) -> ExitStatus {
    let module_arg = format!("{prefix}/{module_name}.wasm", prefix = PATH_PREFIX);
    Command::new("./target/debug/wasmtime")
        .arg("run")
        .arg("--wali")
        .arg("-W")
        .arg("unknown-imports-trap=y")
        .arg(module_arg)
        .env("WASMTIME_LOG", "off")
        .stdout(Stdio::null())
        .spawn()
        .expect("failed to spawn process")
        .wait()
        .expect("failed to wait on child")
}

#[test]
fn socket() {
    let server_join = std::thread::spawn(|| run_wali_module("socket_server"));

    std::thread::sleep(std::time::Duration::from_millis(500));
    let client_join = std::thread::spawn(|| run_wali_module("socket_client"));

    let server_exit_status = server_join.join().unwrap();
    let client_exit_status = client_join.join().unwrap();

    assert!(server_exit_status.success());
    assert!(client_exit_status.success());
}

macro_rules! wali_test {
    ($test_name: literal) => {
        paste::item! {
            #[test]
            fn [<$test_name>]() {
                wali_test($test_name);
            }
        }
    };
}

wali_test!("access");
wali_test!("args");
wali_test!("base");
wali_test!("clock_gettime");
wali_test!("clock_nanosleep");
wali_test!("math");
wali_test!("mmap");
wali_test!("mmap2"); // Needs the file 'sample_text.txt' in the directory where it is run
wali_test!("mprotect");
// wali_test!("nanosleep"); // Exit status 1 -- also for the iwasm version
wali_test!("printf");
wali_test!("sizes");
wali_test!("uname");
wali_test!("va_args");
wali_test!("write");
wali_test!("wprintf");
wali_test!("epoll");
wali_test!("fn_ptr");
wali_test!("fn_ptr_simple");
wali_test!("getenv");
wali_test!("malloc");
wali_test!("msghdr");
wali_test!("platform");
wali_test!("fileops");
wali_test!("noflock");
wali_test!("flock");
wali_test!("lseek");
wali_test!("setpgid");
// wali_test!("getdents64"); // does not find the file
wali_test!("kill");
wali_test!("alarm");
wali_test!("pipe");
wali_test!("rawfork");
wali_test!("sigprocmask");
wali_test!("fork");
wali_test!("dup");
wali_test!("fcntl");
wali_test!("execve");
wali_test!("thread");
wali_test!("access_thread");
wali_test!("simple_thread");
wali_test!("safe_thread");
wali_test!("statall");
// wali_test!("statfs"); // lacks a file (same failure for the iwasm test)
// wali_test!("fstat"); // fails since no proper exit group implementation atm
// wali_test!("fstat2"); // fails since no proper exit group implementation atm
// wali_test!("fstatfs"); // fails since no proper exit group implementation atm
