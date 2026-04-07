#![feature(rustc_private)]
#![deny(clippy::arithmetic_side_effects)]

extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

use errgonomic::{eprintln_error, handle, handle_opt};
use futures::stream::{FuturesUnordered, StreamExt};
use rustc_driver::{Callbacks, Compilation};
use rustc_hir::def::DefKind;
use rustc_hir::def_id::LocalDefId;
use rustc_middle::ty::TyCtxt;
use rustc_session::{EarlyDiagCtxt, config::ErrorOutputType};
use rustc_span::SpanSnippetError;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::io;
use std::path::{Path, PathBuf};
use tempfile::Builder;
use thiserror::Error;
use tokio::runtime::Builder as RuntimeBuilder;

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
        match run_after_analysis(tcx) {
            Ok((output_dir, created_files)) => {
                println!("wrote {created_files} type files to {}", output_dir.display());
            }
            Err(error) => {
                eprintln_error(&error);
            }
        }

        // Stop compilation before code generation
        Compilation::Stop
    }
}

fn run_after_analysis(tcx: TyCtxt) -> Result<(PathBuf, usize), RunAfterAnalysisError> {
    use RunAfterAnalysisError::*;
    let type_ids = handle!(collect_type_ids(tcx), CollectTypeIdsFailed);
    let output_dir = handle!(create_output_dir(), CreateOutputDirFailed);
    let runtime = handle!(RuntimeBuilder::new_current_thread().enable_all().build(), BuildFailed);
    let created_files = handle!(runtime.block_on(write_type_files(tcx, type_ids, output_dir.as_path())), WriteTypeFilesFailed);
    Ok((output_dir, created_files))
}

#[allow(clippy::question_mark)]
fn collect_type_ids(tcx: TyCtxt<'_>) -> Result<Vec<LocalDefId>, CollectTypeIdsError> {
    use CollectTypeIdsError::*;
    let type_ids = tcx
        .iter_local_def_id()
        .filter(|local_def_id| type_kind(tcx.def_kind(local_def_id.to_def_id())).is_some())
        .collect::<Vec<_>>();

    let duplicate_result = type_ids
        .iter()
        .try_fold(HashMap::<String, String>::new(), |mut seen_paths, local_def_id| {
            let def_id = local_def_id.to_def_id();
            let file_name = type_file_name(tcx, *local_def_id);
            let type_path = tcx.def_path_str(def_id);
            match seen_paths.entry(file_name.clone()) {
                Entry::Vacant(entry) => {
                    entry.insert(type_path);
                    Ok(seen_paths)
                }
                Entry::Occupied(entry) => Err(DuplicateFileNameInvalid {
                    file_name,
                    first_type_path: entry.get().clone(),
                    second_type_path: type_path,
                }),
            }
        });

    if let Err(source) = duplicate_result {
        return Err(source);
    }

    Ok(type_ids)
}

fn create_output_dir() -> Result<PathBuf, CreateOutputDirError> {
    use CreateOutputDirError::*;
    let temp_dir = handle!(Builder::new().prefix("baselang-spec-").tempdir(), TempdirFailed);
    Ok(temp_dir.keep())
}

async fn write_type_files<'tcx>(tcx: TyCtxt<'tcx>, type_ids: Vec<LocalDefId>, output_dir: &Path) -> Result<usize, WriteTypeFilesError> {
    use WriteTypeFilesError::*;
    let mut write_futures = type_ids
        .into_iter()
        .map(|local_def_id| write_type_file(tcx, local_def_id, output_dir))
        .collect::<FuturesUnordered<_>>();
    let mut created_files = 0usize;

    while let Some(result) = write_futures.next().await {
        let _ = handle!(result, WriteTypeFileFailed);
        created_files = handle_opt!(created_files.checked_add(1usize), CheckedAddFailed, created_files);
    }

    Ok(created_files)
}

async fn write_type_file<'tcx>(tcx: TyCtxt<'tcx>, local_def_id: LocalDefId, output_dir: &Path) -> Result<String, WriteTypeFileError> {
    use WriteTypeFileError::*;
    let def_id = local_def_id.to_def_id();
    let file_name = type_file_name(tcx, local_def_id);
    let path = output_dir.join(file_name);
    let type_path = tcx.def_path_str(def_id);

    // This forces the future to suspend before using `tcx`, proving the local
    // current-thread runtime can resume it with the same compiler context.
    tokio::task::yield_now().await;

    let type_source = handle!(type_source_code(tcx, local_def_id), TypeSourceCodeFailed);
    handle!(tokio::fs::write(&path, type_source).await, WriteFailed, path, type_path);

    Ok(type_path)
}

fn type_source_code(tcx: TyCtxt<'_>, local_def_id: LocalDefId) -> Result<String, TypeSourceCodeError> {
    use TypeSourceCodeError::*;
    let def_id = local_def_id.to_def_id();
    let type_path = tcx.def_path_str(def_id);
    let hir_id = tcx.local_def_id_to_hir_id(local_def_id);
    let item_span = tcx.hir_span_with_body(hir_id);
    let source_span = if item_span.from_expansion() { item_span.source_callsite() } else { item_span };
    let source_map = tcx.sess.source_map();
    match source_map.span_to_snippet(source_span) {
        Ok(type_source) => Ok(type_source),
        Err(reason) => Err(SpanToSnippetFailed {
            reason: Box::new(reason),
            type_path,
        }),
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

fn type_file_name(tcx: TyCtxt<'_>, local_def_id: LocalDefId) -> String {
    let def_id = local_def_id.to_def_id();
    let type_name = tcx.item_name(def_id);
    format!("{type_name}.rs")
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

#[allow(clippy::enum_variant_names)]
#[derive(Error, Debug)]
enum RunAfterAnalysisError {
    #[error("failed to collect type definitions")]
    CollectTypeIdsFailed { source: Box<CollectTypeIdsError> },
    #[error("failed to create the output directory")]
    CreateOutputDirFailed { source: Box<CreateOutputDirError> },
    #[error("failed to build the current-thread runtime")]
    BuildFailed { source: io::Error },
    #[error("failed to write type files")]
    WriteTypeFilesFailed { source: Box<WriteTypeFilesError> },
}

#[derive(Error, Debug)]
enum CollectTypeIdsError {
    #[error("type file name '{file_name}' maps to both '{first_type_path}' and '{second_type_path}'")]
    DuplicateFileNameInvalid { file_name: String, first_type_path: String, second_type_path: String },
}

#[derive(Error, Debug)]
enum CreateOutputDirError {
    #[error("failed to create a temporary output directory")]
    TempdirFailed { source: io::Error },
}

#[derive(Error, Debug)]
enum WriteTypeFilesError {
    #[error("failed to increment created file count from {created_files}")]
    CheckedAddFailed { created_files: usize },
    #[error("failed to write a type file")]
    WriteTypeFileFailed { source: Box<WriteTypeFileError> },
}

#[derive(Error, Debug)]
enum WriteTypeFileError {
    #[error("failed to read source code for a type")]
    TypeSourceCodeFailed { source: Box<TypeSourceCodeError> },
    #[error("failed to write type '{type_path}' to '{}'", path.display())]
    WriteFailed { source: io::Error, path: PathBuf, type_path: String },
}

#[derive(Error, Debug)]
enum TypeSourceCodeError {
    #[error("failed to extract source code for type '{type_path}': {reason:?}")]
    SpanToSnippetFailed { reason: Box<SpanSnippetError>, type_path: String },
}
