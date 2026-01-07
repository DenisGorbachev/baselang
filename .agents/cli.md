# CLI guidelines

## Dependencies

- `clap` (features: at least "derive", "env")
- `tokio` (features: at least "macros", "rt", "rt-multi-thread")
- `cli-utils`

## File layout and required items

### `src/main.rs`

- Must declare the entrypoint using the `cli_utils::main!` macro

### `src/command.rs`

- Must define a [command-like struct](#command-like-struct) named `Command`
- Must define a [subcommand-like enum](#subcommand-like-enum) named `Subcommand`
- Must contain `cli_utils::test!(Command);` (this expands to a test for `Command` using `debug_assert` from `clap`)

## Definitions

### Command-like struct

A struct that contains fields for CLI arguments.

- Must have a name that ends with `Command`
- Must derive `Parser` from `clap`
- Must be attached as a child module to the parent command struct (or src/lib.rs if it's a top-level `Command`)
- May contain a `subcommand` field annotated with `#[command(subcommand)]`
- Must have a `pub async fn run`
  - Must return a `Result`

Example: see src/command.rs

### Subcommand-like enum

An enum that contains variants for CLI subcommands.

- Must have a name that ends with `Subcommand`
- Must derive `Subcommand` from `clap`
- Must be located in the same file as its parent command struct
- Each variant must be a tuple variant containing exactly one subcommand
- Must have a `pub async fn run`
  - Must match on `self` and call `run` on each subcommand
  - Must return a `Result`

Example: see src/command.rs
