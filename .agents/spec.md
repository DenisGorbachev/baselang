# Baselang concepts

## `.mise/tasks/spec/fix.sh`

* Must run `cargo build --release --manifest-path spec/Cargo.toml` to build the `spec` package
* Must run `cargo fix` to check the primary package using the rustc wrapper from the `spec` package
  * Must set env vars:
    * `RUSTC_WORKSPACE_WRAPPER` (set it to the path to `spec` package main binary, which should be already built)
