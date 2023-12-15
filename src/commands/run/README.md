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