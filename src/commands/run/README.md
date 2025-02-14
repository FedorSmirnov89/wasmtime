# WALI Extension of Wasmtime

Readme file for documenting the state of the integration of WALI into Wasmtime.

## Build

After cloning the repository, clone the crates that wasmtime depends on by running

```
git submodule update --init
```

After that, build the runtime to check that evth works:

```
cargo b
```

### Note on Linking

During the build process, you may run out of memory, so that the OS will kill the `cargo build` process (this happened a lot on my machine). For me, using a different linker solved the problem. You can configure cargo to use `rust-lld` by adding the following to `.cargo/config`:

```
[target.x86_64-unkown-linux]
linker = "rust-lld"
```

(you will have to use the appropriate target triple for your machine, you can look it up using `rustc --print cfg`)

## Running a module

We will assume that you have a WASM module which was compiled against the WALI interaface. See the documentation in the [Wali Repository](https://github.com/arjunr2/WALI) for information about the compilation.

To run the module with Wasmtime, run

```
./target/debug/wasmtime run --wali -W unknown-imports-trap=y [path_to_wasm_file]
```

(we trap for unknown imports for now, since a large fraction of the host function required by WALI is not there yet).

## Logging

We are using the tracing-based logging infrastructure of Wasmtime for the logging within the Wali code. To enable logging of messages of the `wali` module and its submodules, set the corresponding environment variable when running the run command like so:

```
WASMTIME_LOG=wasmtime_cli::commands::run::wali=[error|warn|info|debug|trace] [run_command]
```

## Testing

### Syscall tests

The currently supported syscalls are tested with unit tests (although they are implemented more as integration tests (in an ugly way assuming that the runtime is compiled when the tests are running) -- this will be implemented properly when the wali stuff is moved into an own crate) within the `commands::run::wali` module. Each of these tests uses the wali-extended runtime to run a test WASM module from the [Wali test suite](https://github.com/arjunr2/WALI/tree/main/tests). 

To run the tests:

1. Check out the Wali repository somewhere on your system and follow the steps in its readme to build the test modules
2. Create the file `path_prefix.txt` in the `local` folder of this directory (`local` is in the gitignore, so you will hava to create the directory). The file should contain the file path prefix of the tests (For instance, if the test WASM files are in the `/home/wali/WALI/tests/wasm` directory on your system, this should be the exact content of the file -- no formatting, no single/double quotes -- the content of this file is read in as a string during the compilation of the tests).
3. Run the wali tests by running `cargo test --lib commands::run::wali`

## Implementation Progress

Currently, the implementation focus is on implementing the functionality necessary to pass the test suite defined in the [Wali repo](https://github.com/arjunr2/WALI/tree/main/tests). The progress in this is detailed below:

### Passing (automated) Tests
- access.wasm
- access_thread.wasm
- alarm.wasm
- args.wasm
- base.wasm
- clock_gettime.wasm
- clock_nanosleep.wasm
- dup.wasm
- epoll.wasm
- execve.wasm
- fcntl.wasm
- fileops.wasm
- flock.wasm
- fn_ptr.wasm
- fn_ptr_simple.wasm
- fork.wasm
- getenv.wasm 
- kill.wasm
- lseek.wasm
- malloc.wasm
- math.wasm
- mmap.wasm
- mprotect.wasm
- msghdr.wasm
- noflock.wasm
- pipe.wasm
- platform.wasm 
- printf.wasm
- rawfork.wasm
- safe_thread.wasm
- setpgid.wasm
- sigprocmask.wasm
- simple_thread.wasm
- sizes.wasm
- socket_client.wasm
- socket_server.wasm
- statall.wasm
- thread.wasm
- uname.wasm
- va_args.wasm
- write.wasm
- wprintf.wasm


### Output seems to be okay, but exiting with sth other than 0
- mmap2.wasm -- exit status 1
- nanosleep.wasm -- exit status 1
- getdirents.wasm -- missing file
- statfs.wasm -- missing file
- fstat.wasm -- needs proper exit group management
- fstat2.wasm -- needs proper exit group management
- fstatfs.wasm -- needs proper exit group management

### Not Yet Implemented/Tested
- alarm_signal.wasm -- needs rt_sigaction
- exit.wasm -- needs sys_exit_group
- futex_stop.wasm -- needs rt_sigaction
- infinite_loop.wasm -- seems infinite alright :) not sure what the intended behavior is
- loop.wasm -- needs rt_sigaction
- lstat.wasm -- needs exit_group
- raise.wasm -- needs rt_sigaction
- sigaltstack.wasm -- needs sigaltstack
- signal.wasm -- needs rt_sigaction
- signal2.wasm -- needs rt_sigaction
- signal3.wasm -- needs rt_sigaction
- sigsuspend.wasm -- needs rt_sigaction
- sleep_kill.wasm -- hangs after calling nanosleep (same when run with iwasm)
- stat.wasm -- needs exit group
- streamin.wasm -- not sure what this one is missing; check on it later
- utime.wasm -- needs sys_exit_group