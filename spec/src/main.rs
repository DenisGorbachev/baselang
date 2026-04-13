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
use rustc_hir::def_id::LocalDefId;
use rustc_middle::ty::{self, AdtDef, FieldDef, TyCtxt};
use rustc_session::{EarlyDiagCtxt, config::ErrorOutputType};
use rustc_span::{Symbol, sym};
use spec::{Ctx, Field, Outcome, var_struct_must_have_field_constructors_of_option_vec};
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
        let ctx = Ctx::from(tcx);
        self.0 = Some(SyntacticTestReport::generate(ctx));
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
    pub fn generate(ctx: Ctx<'_>) -> Result<Self, ReportGenerateError> {
        use ReportGenerateError::*;
        let runtime = handle!(RuntimeBuilder::new_current_thread().enable_all().build(), BuildFailed);
        Ok(runtime.block_on(Self::gather(&ctx)))
    }

    pub async fn gather(ctx: &Ctx<'_>) -> Self {
        tokio::task::yield_now().await;
        let struct_var = StructVar::gather(ctx);
        Self {
            struct_var,
        }
    }
}

#[derive(Facet, Debug)]
pub struct StructVar {
    pub is_present: Outcome,
    pub is_unique: Outcome,
    pub fields: StructVarFields,
}

impl StructVar {
    pub fn gather(_ctx: &Ctx<'_>) -> Self {
        // use StructVarReportedError::*;
        todo!()
        // match matches.as_slice() {
        //     [] => Self {
        //         is_present: Fail,
        //         is_unique: Fail,
        //         type_paths: Vec::new(),
        //         reported_error: must_report_error.then_some(NotFound),
        //         fields: StructVarFields::empty(),
        //     },
        //     [(var_struct_def_id, type_path)] => Self::gather_unique(ctx, *var_struct_def_id, type_path),
        //     _ => Self {
        //         is_present: Pass,
        //         is_unique: Fail,
        //         type_paths: matches
        //             .into_iter()
        //             .map(|(_local_def_id, type_path)| type_path)
        //             .collect(),
        //         reported_error: must_report_error.then_some(MultipleFound),
        //         fields: StructVarFields::empty(),
        //     },
        // }
    }

    // pub fn gather_unique(ctx: &Ctx<'_>, var_struct_def_id: LocalDefId, type_path: &str) -> Self {
    //     use StructVarReportedError::*;
    //     let must_report_error = must_report_var_struct_constructors_field();
    //     let var_type = ctx
    //         .type_of(var_struct_def_id.to_def_id())
    //         .instantiate_identity();
    //     match var_type.ty_adt_def() {
    //         Some(var_adt) => Self {
    //             is_present: Pass,
    //             is_unique: Pass,
    //             type_paths: vec![type_path.to_owned()],
    //             reported_error: None,
    //             fields: StructVarFields::gather(ctx, var_adt),
    //         },
    //         None => Self {
    //             is_present: Pass,
    //             is_unique: Pass,
    //             type_paths: vec![type_path.to_owned()],
    //             reported_error: must_report_error.then_some(TypeInvalid),
    //             fields: StructVarFields::empty(),
    //         },
    //     }
    // }
}

#[derive(Facet, Debug)]
#[repr(u8)]
pub enum StructVarReportedError {
    NotFound,
    MultipleFound,
    TypeInvalid,
}

#[derive(Facet, Debug)]
#[repr(u8)]
pub enum StructVarFields {
    StructVarFieldsWithConstructors { constructors: StructVarFieldsConstructorsOptionVec },
    StructVarFieldsWithoutConstructors {},
}

impl StructVarFields {
    /// `var` must be a struct
    pub fn gather<'tcx>(ctx: &Ctx<'tcx>, var: AdtDef<'tcx>) -> Self {
        if var_struct_must_have_field_constructors_of_option_vec() == Some(true) {
            let constructors = StructVarFieldsConstructorsOptionVec::gather(ctx, var);
            Self::StructVarFieldsWithConstructors {
                constructors,
            }
        } else {
            Self::StructVarFieldsWithoutConstructors {}
        }
    }
}

#[derive(Facet, Default, Debug)]
pub struct StructVarFieldsConstructorsOptionVec {
    pub is_present: Outcome,
    pub has_type_option_vec_var: Outcome,
}

impl StructVarFieldsConstructorsOptionVec {
    fn gather<'tcx>(ctx: &Ctx<'tcx>, var: AdtDef<'tcx>) -> Self {
        let constructors_field_name = Symbol::intern("constructors");
        let constructors_field = var.all_fields().find(|x| x.name == constructors_field_name);
        if let Some(constructors_field_def) = constructors_field {
            let is_present = Pass;
            let has_type_option_vec_var = Self::has_type_option_vec_var(ctx, var, constructors_field_def);
            Self {
                is_present,
                has_type_option_vec_var,
            }
        } else {
            Self::default()
        }
    }

    fn has_type_option_vec_var<'tcx>(ctx: &Ctx<'tcx>, var: AdtDef<'tcx>, constructors_field: &'tcx FieldDef) -> Outcome {
        let Some(var_struct_def_id) = var.did().as_local() else {
            return Fail;
        };
        let field = Field::new(*ctx.tcx(), constructors_field);
        let field_type = field.ty();
        if is_option_vec_of_local_adt(field.tcx, field_type, var_struct_def_id) { Pass } else { Fail }
    }
}

// impl StructVarFieldsConstructorsOptionVec {
//     pub fn empty() -> Self {
//         Self {
//             must_be_option_vec_var: var_struct_must_have_field_constructors_of_option_vec(),
//             is_present: false,
//             actual_type: None,
//             has_type_option_vec_var: false,
//             reported_error: None,
//         }
//     }
//
//     pub fn gather(ctx: &Ctx<'_>, var_struct_def_id: LocalDefId, constructors_field: Option<&FieldDef>) -> Self {
//         use StructVarFieldsConstructorsReportedError::*;
//         let must_be_option_vec_var = var_struct_must_have_field_constructors_of_option_vec();
//         let must_report_error = must_be_option_vec_var == Some(true);
//         match constructors_field {
//             Some(constructors_field) => {
//                 let field = Field::new(*ctx.tcx(), constructors_field);
//                 let field_type = field.ty();
//                 let actual_type = field_type.to_string();
//                 let has_type_option_vec_var = is_option_vec_of_local_adt(field.tcx, field_type, var_struct_def_id);
//                 let reported_error = if has_type_option_vec_var || !must_report_error { None } else { Some(TypeInvalid) };
//                 Self {
//                     must_be_option_vec_var,
//                     is_present: true,
//                     actual_type: Some(actual_type),
//                     has_type_option_vec_var,
//                     reported_error,
//                 }
//             }
//             None => Self {
//                 must_be_option_vec_var,
//                 is_present: false,
//                 actual_type: None,
//                 has_type_option_vec_var: false,
//                 reported_error: must_report_error.then_some(NotFound),
//             },
//         }
//     }
// }

// #[derive(Facet, Default, Debug)]
// struct StructVarFieldsConstructorsAbsent {
//     is_absent: Outcome,
// }
//
// impl StructVarFieldsConstructorsAbsent {
//     pub fn gather(_ctx: &Ctx<'_>) -> Self {
//         todo!()
//     }
// }

pub fn is_option_vec_of_local_adt(tcx: TyCtxt<'_>, field_type: ty::Ty<'_>, local_adt_def_id: LocalDefId) -> bool {
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
    matches!(inner_type.kind(), ty::Adt(var_def, _) if var_def.did() == local_adt_def_id.to_def_id())
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
