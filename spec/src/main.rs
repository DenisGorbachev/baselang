#![deny(clippy::arithmetic_side_effects)]
#![deny(unused_crate_dependencies)]

use clap::Parser;
use errgonomic::exit_result;
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    if is_rustc_wrapper_mode() {
        let result = run_rustc_wrapper_from_env();
        exit_result(result)
    } else {
        let args = Command::parse();
        let result = args.run().await;
        exit_result(result)
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Command::command().debug_assert();
}

mod command;

pub use command::*;

mod functions;

pub use functions::*;

mod types;

pub use types::*;
