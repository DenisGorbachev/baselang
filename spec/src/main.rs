#![feature(rustc_private)]
#![deny(clippy::arithmetic_side_effects)]

extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;

use rustc_driver::{Callbacks, Compilation};
use rustc_hir::def::DefKind;
use rustc_hir::def_id::LocalDefId;
use rustc_middle::ty::TyCtxt;
use rustc_session::{EarlyDiagCtxt, config::ErrorOutputType};

/// This executable must only be called as a `RUSTC_WORKSPACE_WRAPPER` with `cargo fix --package baselang --lib`
fn main() {
    let early_dcx = EarlyDiagCtxt::new(ErrorOutputType::default());
    rustc_driver::init_rustc_env_logger(&early_dcx);
    rustc_driver::install_ice_hook(rustc_driver::DEFAULT_BUG_REPORT_URL, |_| ());
    rustc_driver::install_ctrlc_handler();
    let raw_args = rustc_driver::args::raw_args(&early_dcx);
    // compiler_args must not contain raw_args[0], which is the path to the current executable
    let compiler_args = &raw_args[1..];
    let mut visitor = Visitor {};
    // catch_with_exit_code converts rustc fatal-error unwinds into the expected compiler exit code.
    let exit_code = rustc_driver::catch_with_exit_code(|| rustc_driver::run_compiler(compiler_args, &mut visitor));
    std::process::exit(exit_code);
}

struct Visitor {}

impl Callbacks for Visitor {
    fn after_analysis<'tcx>(&mut self, _: &rustc_interface::interface::Compiler, tcx: TyCtxt<'tcx>) -> Compilation {
        let lines = tcx
            .iter_local_def_id()
            .filter_map(|local_def_id| type_line(tcx, local_def_id));

        for line in lines {
            println!("{line}")
        }

        // Stop compilation before code generation
        Compilation::Stop
    }
}

/// Unused, kept for historical purposes
pub fn has_crate_type(compiler_args: &[String], expected_crate_type: &str) -> bool {
    let mut iter = compiler_args
        .iter()
        .skip_while(|arg| *arg != "--crate-type");
    // skip the "--crate-type" itself
    iter.next();
    let crate_type_opt = iter.next();
    crate_type_opt
        .map(|actual_crate_type| actual_crate_type == expected_crate_type)
        .unwrap_or_default()
}

fn type_line(tcx: TyCtxt<'_>, local_def_id: LocalDefId) -> Option<String> {
    let def_id = local_def_id.to_def_id();
    type_kind(tcx.def_kind(def_id)).map(|type_kind| format!("{type_kind} {}", tcx.def_path_str(def_id)))
}

fn type_kind(def_kind: DefKind) -> Option<&'static str> {
    use DefKind::*;
    match def_kind {
        Enum => Some("enum"),
        Struct => Some("struct"),
        TyAlias => Some("type"),
        Union => Some("union"),
        _ => None,
    }
}
