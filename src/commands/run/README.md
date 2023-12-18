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
- args.wasm
- base.wasm
- clock_gettime.wasm
- clock_nanosleep.wasm
- math.wasm
- mmap.wasm
- mprotect.wasm
- printf.wasm
- sizes.wasm
- uname.wasm
- va_args.wasm
- write.wasm
- wprintf.wasm

### Output seems to be okay, but 1 as exit status
- mmap2.wasm
- nanosleep.wasm

### Not Yet Implemented/Tested
- access_thread.wasm
- alarm.wasm -- needs fork
- alarm_signal.wasm
- dup.wasm
- epoll.wasm
- execve.wasm
- exit.wasm -- needs sys_exit_group
- fcntl.wasm
- fileops.wasm
- flock.wasm
- fn_ptr.wasm
- fn_ptr_simple.wasm
- fork.wasm
- fstat.wasm
- fstat2.wasm
- fstatfs.wasm
- futex_stop.wasm
- getdirents.wasm
- getenv.wasm
- infinite_loop.wasm
- kill.wasm
- loop.wasm
- lseek.wasm
- lstat.wasm
- malloc.wasm
- msghdr.wasm
- noflock.wasm
- pipe.wasm
- platform.wasm
- raise.wasm
- rawfork.wasm
- safe_thread.wasm
- setpgid.wasm
- sigaltstack.wasm
- signal.wasm
- signal2.wasm
- signal3.wasm
- sigprocmask.wasm
- sigsuspend.wasm
- simple_thread.wasm
- sleep_kill.wasm
- socket_client.wasm
- socket_server.wasm
- stat.wasm
- statall.wasm
- statfs.wasm
- streamin.wasm
- thread.wasm
- utime.wasm