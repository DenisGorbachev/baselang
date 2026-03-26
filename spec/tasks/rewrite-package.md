# Rewrite the `spec` package

## Tasks

* Rewrite the `spec` package according to concepts below
* Add a [`fix`](#misetasksspecfixsh) task to main package
* Ensure `mise run spec:fix` correctly lists the types in the main package

## Concepts

### `spec/src/main.rs`

* Must work as a rustc wrapper that is run by `cargo`
* Must not call `is_rustc_wrapper_mode`
* Must call `rustc_driver::run_compiler`
  * Must pass through the flags that `cargo` passes in order to correctly build the package
* Must contain a `struct Visitor`
  * Must implement `Callbacks` from `rustc_driver`
    * Must list the type names in `TyCtxt` to stdout
* Must not use `syn`
* Must not contain unused code

### `.mise/tasks/spec/fix.sh`

* Must run `cargo build --release --manifest-path spec/Cargo.toml` to build the `spec` package
* Must run `cargo fix` to check the primary package using the rustc wrapper from the `spec` package
  * Must set env vars:
    * `RUSTC_WORKSPACE_WRAPPER` (set it to the path to `spec` package main binary, which should be already built)
