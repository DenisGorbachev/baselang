use clap::Parser;
use errgonomic::exit_result;
use spec::Command;
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    exit_result(Command::parse().run().await)
}
