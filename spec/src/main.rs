use clap::Parser;
use errgonomic::exit_result;
use spec::{Command, is_rustc_wrapper_mode, run_rustc_wrapper_from_env};
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
