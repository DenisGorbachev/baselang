#![feature(rustc_private)]
#![deny(clippy::arithmetic_side_effects)]

extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

use Outcome::*;
use errgonomic::{exit_result, handle, handle_opt};
use facet::Facet;
use facet_pretty::{FacetPretty, PrettyPrinter};
use rustc_driver::{Callbacks, Compilation};
use rustc_hir::def::DefKind;
use rustc_hir::def_id::LocalDefId;
use rustc_middle::ty::{self, FieldDef, TyCtxt};
use rustc_session::{EarlyDiagCtxt, config::ErrorOutputType};
use rustc_span::{Symbol, sym};
use spec::{Outcome, var_struct_must_have_field_constructors_of_option_vec};
use std::io;
use std::process::ExitCode;
use thiserror::Error;
use tokio::runtime::Builder as RuntimeBuilder;

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
        let printer = PrettyPrinter::default()
            .with_doc_comments(true)
            .with_minimal_option_names(true);
        let display = report.pretty_with(printer);
        println!("{display}");
    }
    Ok(ExitCode::SUCCESS)
}

#[derive(Debug, Default)]
pub struct Visitor(pub Option<Result<SyntacticTestReport, ReportGenerateError>>);

impl Callbacks for Visitor {
    fn after_analysis<'tcx>(&mut self, _: &rustc_interface::interface::Compiler, tcx: TyCtxt<'tcx>) -> Compilation {
        self.0 = Some(SyntacticTestReport::generate(tcx));
        // Stop compilation before code generation
        Compilation::Stop
    }
}

#[derive(Facet, Debug)]
pub struct SyntacticTestReport {
    /// `struct Var`
    pub struct_var: StructVar,
}

impl SyntacticTestReport {
    pub fn generate(tcx: TyCtxt<'_>) -> Result<Self, ReportGenerateError> {
        use ReportGenerateError::*;
        let runtime = handle!(RuntimeBuilder::new_current_thread().enable_all().build(), BuildFailed);
        Ok(runtime.block_on(Self::gather(tcx)))
    }

    pub async fn gather(tcx: TyCtxt<'_>) -> Self {
        let struct_var = StructVar::gather(tcx).await;
        Self {
            struct_var,
        }
    }
}

#[derive(Facet, Debug)]
pub struct StructVar {
    pub is_present: Outcome,
    pub is_unique: Outcome,
    pub type_paths: Vec<String>,
    pub reported_error: Option<StructVarReportedError>,
    pub fields: StructVarFields,
}

impl StructVar {
    pub async fn gather(tcx: TyCtxt<'_>) -> Self {
        use StructVarReportedError::*;
        tokio::task::yield_now().await;
        let must_report_error = must_report_var_struct_constructors_field();
        let matches = find_struct(tcx, "Var")
            .map(|local_def_id| (local_def_id, tcx.def_path_str(local_def_id)))
            .collect::<Vec<_>>();

        match matches.as_slice() {
            [] => Self {
                is_present: Fail,
                is_unique: Fail,
                type_paths: Vec::new(),
                reported_error: must_report_error.then_some(NotFound),
                fields: StructVarFields::empty(),
            },
            [(var_struct_def_id, type_path)] => Self::gather_unique(tcx, *var_struct_def_id, type_path),
            _ => Self {
                is_present: Pass,
                is_unique: Fail,
                type_paths: matches
                    .into_iter()
                    .map(|(_local_def_id, type_path)| type_path)
                    .collect(),
                reported_error: must_report_error.then_some(MultipleFound),
                fields: StructVarFields::empty(),
            },
        }
    }

    pub fn gather_unique(tcx: TyCtxt<'_>, var_struct_def_id: LocalDefId, type_path: &str) -> Self {
        use StructVarReportedError::*;
        let must_report_error = must_report_var_struct_constructors_field();
        let var_type = tcx
            .type_of(var_struct_def_id.to_def_id())
            .instantiate_identity();
        match var_type.ty_adt_def() {
            Some(var_adt) => Self {
                is_present: Pass,
                is_unique: Pass,
                type_paths: vec![type_path.to_owned()],
                reported_error: None,
                fields: StructVarFields::gather(tcx, var_struct_def_id, var_adt.non_enum_variant().fields.iter()),
            },
            None => Self {
                is_present: Pass,
                is_unique: Pass,
                type_paths: vec![type_path.to_owned()],
                reported_error: must_report_error.then_some(TypeInvalid),
                fields: StructVarFields::empty(),
            },
        }
    }
}

#[derive(Facet, Debug)]
#[repr(u8)]
pub enum StructVarReportedError {
    NotFound,
    MultipleFound,
    TypeInvalid,
}

#[derive(Facet, Debug)]
pub struct StructVarFields {
    pub actual_fields: Vec<String>,
    pub constructors: StructVarFieldsConstructors,
}

impl StructVarFields {
    pub fn empty() -> Self {
        Self {
            actual_fields: Vec::new(),
            constructors: StructVarFieldsConstructors::empty(),
        }
    }

    pub fn gather<'a>(tcx: TyCtxt<'_>, var_struct_def_id: LocalDefId, fields: impl IntoIterator<Item = &'a FieldDef>) -> Self {
        let fields = fields.into_iter().collect::<Vec<_>>();
        let actual_fields = fields
            .iter()
            .map(|field| {
                let field_type = tcx.type_of(field.did).instantiate_identity();
                format!("{}: {field_type}", field.name)
            })
            .collect();
        let constructors_field = fields
            .iter()
            .copied()
            .find(|field| field.name == Symbol::intern("constructors"));
        Self {
            actual_fields,
            constructors: StructVarFieldsConstructors::gather(tcx, var_struct_def_id, constructors_field),
        }
    }
}

#[derive(Facet, Debug)]
pub struct StructVarFieldsConstructors {
    pub must_be_option_vec_var: Option<bool>,
    pub is_present: bool,
    pub actual_type: Option<String>,
    pub has_type_option_vec_var: bool,
    pub reported_error: Option<StructVarFieldsConstructorsReportedError>,
}

impl StructVarFieldsConstructors {
    pub fn empty() -> Self {
        Self {
            must_be_option_vec_var: var_struct_must_have_field_constructors_of_option_vec(),
            is_present: false,
            actual_type: None,
            has_type_option_vec_var: false,
            reported_error: None,
        }
    }

    pub fn gather(tcx: TyCtxt<'_>, var_struct_def_id: LocalDefId, constructors_field: Option<&FieldDef>) -> Self {
        use StructVarFieldsConstructorsReportedError::*;
        let must_be_option_vec_var = var_struct_must_have_field_constructors_of_option_vec();
        let must_report_error = must_be_option_vec_var == Some(true);
        match constructors_field {
            Some(constructors_field) => {
                let field_type = tcx.type_of(constructors_field.did).instantiate_identity();
                let actual_type = field_type.to_string();
                let has_type_option_vec_var = is_option_vec_of_var(tcx, field_type, var_struct_def_id);
                let reported_error = if has_type_option_vec_var || !must_report_error { None } else { Some(TypeInvalid) };
                Self {
                    must_be_option_vec_var,
                    is_present: true,
                    actual_type: Some(actual_type),
                    has_type_option_vec_var,
                    reported_error,
                }
            }
            None => Self {
                must_be_option_vec_var,
                is_present: false,
                actual_type: None,
                has_type_option_vec_var: false,
                reported_error: must_report_error.then_some(NotFound),
            },
        }
    }
}

#[derive(Facet, Debug)]
#[repr(u8)]
pub enum StructVarFieldsConstructorsReportedError {
    NotFound,
    TypeInvalid,
}

pub fn find_struct(tcx: TyCtxt<'_>, name: &str) -> impl Iterator<Item = LocalDefId> {
    let item_name = Symbol::intern(name);
    tcx.iter_local_def_id()
        .filter(move |local_def_id| tcx.def_kind(local_def_id.to_def_id()) == DefKind::Struct && tcx.item_name(local_def_id.to_def_id()) == item_name)
}

pub fn must_report_var_struct_constructors_field() -> bool {
    var_struct_must_have_field_constructors_of_option_vec() == Some(true)
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
pub enum RunError {
    #[error("expected rustc wrapper argv to include the current executable path")]
    CompilerArgsMissingInvalid,
    #[error("failed to generate the report")]
    ReportGenerateFailed { source: ReportGenerateError },
}

#[derive(Error, Debug)]
pub enum ReportGenerateError {
    #[error("failed to build the current-thread runtime")]
    BuildFailed { source: io::Error },
}
