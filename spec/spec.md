# Concepts for `spec` package

## `src/main.rs`

* Must work as a rustc wrapper that is run by `cargo`
* Must not call `is_rustc_wrapper_mode`
* Must call `rustc_driver::run_compiler`
  * Must pass through the flags that `cargo` passes in order to correctly build the package
* Must contain a `struct Visitor`
  * Must implement `Callbacks` from `rustc_driver`
* Must not use `syn`
* Must not contain unused code
* Must accept one argument: `error_path`
  * Must not forward this argument to the compiler
