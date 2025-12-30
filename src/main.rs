use baselang::Cli;
use clap::Parser;
use errgonomic::exit_result;
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    let args = Cli::parse();
    let result = args.run().await;
    exit_result(result)
}
