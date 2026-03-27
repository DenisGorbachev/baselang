# Concepts for `spec` package

## `src/main.rs`

* Must work as a rustc wrapper that is run by `cargo`
* Must not call `is_rustc_wrapper_mode`
* Must call `rustc_driver::run_compiler`
  * Must pass through the flags that `cargo` passes in order to correctly build the package
* Must contain a `struct Visitor`
  * Must implement `Callbacks` from `rustc_driver`
    * Must list the type names in `TyCtxt` to stdout
* Must not use `syn`
* Must not contain unused code
