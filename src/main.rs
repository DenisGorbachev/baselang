use baselang::{Cli, Outcome};
use clap::Parser;

#[tokio::main]
async fn main() -> Outcome {
    let args = Cli::parse();
    args.run().await
}
