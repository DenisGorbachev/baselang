#![feature(rustc_private)]
#![deny(clippy::arithmetic_side_effects)]

extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

use aist::Ctx;
use errgonomic::{exit_result, handle, handle_opt};
// use facet_pretty::FacetPretty;
use rustc_driver::{Callbacks, Compilation};
use rustc_middle::ty::TyCtxt;
use rustc_session::{EarlyDiagCtxt, config::ErrorOutputType};
use std::process::ExitCode;
use thiserror::Error;

/// This executable must only be called as a `RUSTC_WORKSPACE_WRAPPER` with `cargo fix --package baselang --lib`
fn main() -> ExitCode {
    exit_result(run())
}

pub fn run() -> Result<ExitCode, RunError> {
    use RunError::*;
    let early_dcx = EarlyDiagCtxt::new(ErrorOutputType::default());
    rustc_driver::init_rustc_env_logger(&early_dcx);
    rustc_driver::install_ice_hook(rustc_driver::DEFAULT_BUG_REPORT_URL, |_| ());
    rustc_driver::install_ctrlc_handler();
    let raw_args = rustc_driver::args::raw_args(&early_dcx);
    let (_current_executable_path, compiler_args) = handle_opt!(raw_args.split_first(), CompilerArgsMissingInvalid);
    let mut visitor = Visitor::default();
    // catch_with_exit_code converts rustc fatal-error unwinds into the expected compiler exit code.
    let compiler_exit_code = rustc_driver::catch_with_exit_code(|| rustc_driver::run_compiler(compiler_args, &mut visitor));
    // TODO: Must stream errors as soon as they are detected
    if compiler_exit_code != 0 {
        std::process::exit(compiler_exit_code)
    }
    if let Some(report_result) = visitor.0 {
        let report = handle!(report_result, ReportGenerateFailed);
        println!("{report:#?}")
        // let printer = PrettyPrinter::default()
        //     .with_doc_comments(true)
        //     .with_minimal_option_names(true);
        // let display = report.pretty_with(printer);
        // println!("{display}");
    }
    Ok(ExitCode::SUCCESS)
}

#[derive(Debug, Default)]
pub struct Visitor(pub Option<Result<SpecTestReport, ReportGenerateError>>);

impl Callbacks for Visitor {
    fn after_analysis<'tcx>(&mut self, _: &rustc_interface::interface::Compiler, tcx: TyCtxt<'tcx>) -> Compilation {
        let ctx = Ctx::from(tcx);
        self.0 = Some(SpecTestReport::new(ctx));
        // Stop compilation before code generation
        Compilation::Stop
    }
}

#[derive(Error, Debug)]
pub enum RunError {
    #[error("expected rustc wrapper argv to include the current executable path")]
    CompilerArgsMissingInvalid,
    #[error("failed to generate the report")]
    ReportGenerateFailed { source: ReportGenerateError },
}

mod syntactic_test_report;

pub use syntactic_test_report::*;
