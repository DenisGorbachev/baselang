# Spec of spec

## Metrics

- Code size
- Prompt length in tokens
- Probability of the agent implementing the spec from the prompt
  - Probability of a one-shot fix ("get errors, notice errors is not empty, ask agent to fix, get errors, notice errors is empty")

## Concepts

### Codex hook

A program that conforms to Codex "Stop" hook specification.

- `reason` field
  - Must include an instruction to read the spec
    - May include a link to the spec
    - May include the full spec as text
      - Must include the specific type names (e.g. `Vec<u32>`)
      - Should render fully qualified paths in the errors
        - Rationale:
          - Some items may have equal names
  - Must include the output of the spec binary (the issues found)
  - May include the following text: "Edit the code in ./src so that all tests pass according to report below. If any of the tests are implemented incorrectly, then you are allowed to edit the code in ./spec and tell me about it, else not allowed. If any of the tests are ambiguous, tell me about it."

### Runner

A program that runs the spec binary.

- Must execute `cargo build`
  - Must set env vars:
    - `RUSTC_BOOTSTRAP=$spec_crate_name`
      - Rationale:
        - Needed to link extern crates (`rustc_*`)
  - Should set flags:
    - `--release`
      - Rationale:
        - The spec binary will be run multiple times by the agent
- Must execute `cargo build`
  - Rationale
    - `build`, not `fix`, because fixes should be applied via `cargo clippy --fix` (which also fixes the issues found reported by clippy)
  - Must set env vars:
    - `RUSTC_WRAPPER=` (empty string) (required to disable any inherited `RUSTC_WRAPPER`, e.g. `sccache`)
    - `RUSTC_WORKSPACE_WRAPPER="$spec_binary_path"`
  - Must set flags:
    - `--manifest-path` to the manifest of the target crate

### File `src/main.rs`

- Must contain [fn main](#fn-main)
- Must contain a `struct Visitor`
  - Must implement `Callbacks` from `rustc_driver`
- Must not use `syn`
- Must not contain unused code

### fn `main`

- Must work as a rustc wrapper that is run by `cargo`
- Must not call `is_rustc_wrapper_mode`
- Must use `rustc_driver` instead of `rustc_interface`
  - Rationale:
    - It must pass `compiler_args` from `cargo`
  - Requirements:
    - Must call helpers:
      - `rustc_driver::init_rustc_env_logger`
      - `rustc_driver::install_ice_hook`
      - `rustc_driver::install_ctrlc_handler`
    - Must call `rustc_driver::run_compiler`
      - Must pass `compiler_args` from `cargo` to build the target crate (it may depend on external crates)
- May output the full specification
- Must write the errors to stdout
  - Should write the errors to stdout as soon as they are discovered
    - Must maintain the context for errors
      - #notes
        - The standard context preservation technique (returning owned vars while bubbling up the error) will not work for streamed errors
          - #notes
            - Context contains the vars from callers
            - Callee doesn't have access to that context
          - #options
            - Print the errors at the top level
            - Pass an ErrorContext into callees
              - Require to register vars as Dyn Debug
                - But that would require holding a ref
            - Do not require to stream errors, allow reporting them collectively
    - #notes
      - This preference may conflict with the "Code size" metric, which has a higher priority

## Result-based report

A struct whose every field is either a `Result` or another [Result-based report](#result-based-report).

- Should not implement `Clone`
  - Rationales:
    - Some errors don't implement `Clone`
      - Examples:
        - `std::io::Error`
- May use `Result<(), E>` instead of `Outcome`
  - Rationales:
    - Some errors contain useful information
      - Examples:
        - `MutatorsFound` contains a list of DefPathStr

### Outcome-based report

A struct whose every field is either [Outcome](#outcome-enum) or another [Outcome-based report](#outcome-based-report).

### Outcome enum

`Outcome` enum from `aist` crate.

### Strict Mono-Result

A `Result` whose `T` = `()` and whose `E` has only a single variant

### Relaxed Mono-Result

A `Result` whose `T` = `()` (no restriction on `E`)
