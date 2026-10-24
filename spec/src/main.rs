use std::process::ExitCode;

fn main() -> ExitCode {
    spec::check().print()
}
