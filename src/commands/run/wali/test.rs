use std::process::{Command, Stdio};

const PATH_PREFIX: &'static str = include_str!("../local/path_prefix.txt");

fn wali_test(test_name: &'static str) {
    let module_arg = format!("{prefix}/{test_name}.wasm", prefix = PATH_PREFIX);

    let exit_status = Command::new("./target/debug/wasmtime")
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
        .expect("failed to wait on child");

    assert!(exit_status.success());
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
// wali_test!("mmap2"); // Exit status 1
wali_test!("mprotect");
// wali_test!("nanosleep"); // Exit status 1
wali_test!("printf");
wali_test!("sizes");
wali_test!("uname");
wali_test!("va_args");
wali_test!("write");
wali_test!("wprintf");
