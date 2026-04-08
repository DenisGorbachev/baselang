#![feature(rustc_private)]
#![deny(clippy::arithmetic_side_effects)]

extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

use errgonomic::{exit_result, handle, handle_opt};
use rustc_driver::{Callbacks, Compilation};
use rustc_hir::def::DefKind;
use rustc_hir::def_id::LocalDefId;
use rustc_middle::ty::{self, TyCtxt};
use rustc_session::{EarlyDiagCtxt, config::ErrorOutputType};
use rustc_span::{Symbol, sym};
use spec::var_struct_must_have_field_constructors_of_option_vec;
use std::io;
use std::process::ExitCode;
use thiserror::Error;
use tokio::runtime::Builder as RuntimeBuilder;

/// This executable must only be called as a `RUSTC_WORKSPACE_WRAPPER` with `cargo fix --package baselang --lib`
fn main() -> ExitCode {
    let early_dcx = EarlyDiagCtxt::new(ErrorOutputType::default());
    rustc_driver::init_rustc_env_logger(&early_dcx);
    rustc_driver::install_ice_hook(rustc_driver::DEFAULT_BUG_REPORT_URL, |_| ());
    rustc_driver::install_ctrlc_handler();
    let raw_args = rustc_driver::args::raw_args(&early_dcx);
    // compiler_args must not contain raw_args[0], which is the path to the current executable
    let compiler_args = &raw_args[1..];
    let mut visitor = Visitor::default();
    // catch_with_exit_code converts rustc fatal-error unwinds into the expected compiler exit code.
    let compiler_exit_code = rustc_driver::catch_with_exit_code(|| rustc_driver::run_compiler(compiler_args, &mut visitor));
    // std::process::exit(compiler_exit_code)
    // TODO: Must stream errors as soon as they are detected
    if compiler_exit_code != 0 {
        std::process::exit(compiler_exit_code)
    }
    let result = visitor.0.expect("inner value must be set");
    exit_result(result.map(|_t| ExitCode::SUCCESS))
}

#[derive(Debug)]
#[derive(Default)]
struct Visitor(Option<Result<Report, ReportGenerateError>>);

impl Callbacks for Visitor {
    fn after_analysis<'tcx>(&mut self, _: &rustc_interface::interface::Compiler, tcx: TyCtxt<'tcx>) -> Compilation {
        self.0 = Some(Report::generate(tcx));
        // Stop compilation before code generation
        Compilation::Stop
    }
}

#[expect(dead_code)]
#[derive(Default, Debug)]
struct Report {
    struct_var: StructVar,
}

impl Report {
    pub fn generate(tcx: TyCtxt<'_>) -> Result<Self, ReportGenerateError> {
        use ReportGenerateError::*;
        let mut report = Self::default();
        let runtime = handle!(RuntimeBuilder::new_current_thread().enable_all().build(), BuildFailed);
        handle!(runtime.block_on(report.gather(tcx)), CheckVarStructConstructorsFieldFailed);
        Ok(report)
    }

    pub async fn gather(&mut self, _tcx: TyCtxt<'_>) -> Result<(), CheckVarStructConstructorsFieldError> {
        todo!()
    }
}

#[expect(dead_code)]
#[derive(Default, Debug)]
struct StructVar {
    is_present: bool,
    has_fields: StructVarFields,
}

#[expect(dead_code)]
#[derive(Default, Debug)]
struct StructVarFields {
    constructors: StructVarFieldsConstructors,
}

#[expect(dead_code)]
#[derive(Default, Debug)]
struct StructVarFieldsConstructors {
    is_present: bool,

    /// Has a type `Option<Vec<Var>>`
    has_type_option_vec_var: bool,
}

pub async fn check_var_struct_constructors_field<'tcx>(tcx: TyCtxt<'tcx>) -> Result<(), CheckVarStructConstructorsFieldError> {
    use CheckVarStructConstructorsFieldError::*;
    tokio::task::yield_now().await;
    if var_struct_must_have_field_constructors_of_option_vec() != Some(true) {
        return Ok(());
    }
    let var_struct_def_id = handle!(find_var_struct_local_def_id(tcx), FindVarStructLocalDefIdFailed);
    let var_type = tcx
        .type_of(var_struct_def_id.to_def_id())
        .instantiate_identity();
    let Some(var_adt) = var_type.ty_adt_def() else {
        return Err(VarStructTypeInvalid {
            type_path: tcx.def_path_str(var_struct_def_id.to_def_id()),
        });
    };
    let type_path = tcx.def_path_str(var_struct_def_id.to_def_id());
    let constructors_field = handle_opt!(
        var_adt
            .non_enum_variant()
            .fields
            .iter()
            .find(|field| field.name == Symbol::intern("constructors")),
        ConstructorsFieldNotFound,
        type_path
    );
    let field_type = tcx.type_of(constructors_field.did).instantiate_identity();
    if is_option_vec_of_var(tcx, field_type, var_struct_def_id) {
        Ok(())
    } else {
        Err(ConstructorsFieldTypeInvalid {
            type_path,
            actual_type: field_type.to_string(),
        })
    }
}

pub fn find_var_struct_local_def_id(tcx: TyCtxt<'_>) -> Result<LocalDefId, FindVarStructLocalDefIdError> {
    use FindVarStructLocalDefIdError::*;
    let var_name = Symbol::intern("Var");
    let var_struct_def_ids = tcx
        .iter_local_def_id()
        .filter(|local_def_id| tcx.def_kind(local_def_id.to_def_id()) == DefKind::Struct)
        .filter(|local_def_id| tcx.item_name(local_def_id.to_def_id()) == var_name)
        .collect::<Vec<_>>();

    match var_struct_def_ids.as_slice() {
        [] => Err(VarStructNotFound),
        [var_struct_def_id] => Ok(*var_struct_def_id),
        _ => Err(MultipleVarStructsInvalid {
            type_paths: var_struct_def_ids
                .into_iter()
                .map(|local_def_id| tcx.def_path_str(local_def_id.to_def_id()))
                .collect(),
        }),
    }
}

pub fn is_option_vec_of_var(tcx: TyCtxt<'_>, field_type: ty::Ty<'_>, var_struct_def_id: LocalDefId) -> bool {
    let ty::Adt(option_def, option_args) = field_type.kind() else {
        return false;
    };
    if !tcx.is_diagnostic_item(sym::Option, option_def.did()) {
        return false;
    }

    let vec_type = option_args.type_at(0);
    let ty::Adt(vec_def, vec_args) = vec_type.kind() else {
        return false;
    };
    if !tcx.is_diagnostic_item(sym::Vec, vec_def.did()) {
        return false;
    }

    let inner_type = vec_args.type_at(0);
    matches!(inner_type.kind(), ty::Adt(var_def, _) if var_def.did() == var_struct_def_id.to_def_id())
}

#[derive(Error, Debug)]
pub enum ReportGenerateError {
    #[error("failed to build the current-thread runtime")]
    BuildFailed { source: io::Error },
    #[error("failed to check whether struct 'Var' defines the required constructors field")]
    CheckVarStructConstructorsFieldFailed { source: Box<CheckVarStructConstructorsFieldError> },
}

#[derive(Error, Debug)]
pub enum CheckVarStructConstructorsFieldError {
    #[error("failed to locate struct 'Var'")]
    FindVarStructLocalDefIdFailed { source: Box<FindVarStructLocalDefIdError> },
    #[error("struct '{type_path}' is not an ADT")]
    VarStructTypeInvalid { type_path: String },
    #[error("struct '{type_path}' must define field 'constructors: Option<Vec<Var>>'")]
    ConstructorsFieldNotFound { type_path: String },
    #[error("struct '{type_path}' must define field 'constructors: Option<Vec<Var>>', found 'constructors: {actual_type}'")]
    ConstructorsFieldTypeInvalid { type_path: String, actual_type: String },
}

#[derive(Error, Debug)]
pub enum FindVarStructLocalDefIdError {
    #[error("struct 'Var' not found")]
    VarStructNotFound,
    #[error("found multiple structs named 'Var': {type_paths:?}")]
    MultipleVarStructsInvalid { type_paths: Vec<String> },
}
