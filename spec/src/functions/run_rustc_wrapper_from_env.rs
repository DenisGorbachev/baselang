use crate::{ActiveCfgs, CollectTypesFromFileError, WrapperInvocation, collect_types_from_file};
use errgonomic::{handle, handle_opt};
use std::collections::hash_map::DefaultHasher;
use std::ffi::OsString;
use std::fs::OpenOptions;
use std::hash::{Hash, Hasher};
use std::io;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::{Command as ProcessCommand, ExitCode, ExitStatus};
use thiserror::Error;

pub const SPEC_WRAPPER_MODE_ENV_VAR: &str = "SPEC_WRAPPER_MODE";
pub const SPEC_SELECTED_PACKAGE_ENV_VAR: &str = "SPEC_SELECTED_PACKAGE";
pub const SPEC_OUTPUT_DIR_ENV_VAR: &str = "SPEC_OUTPUT_DIR";

pub fn is_rustc_wrapper_mode() -> bool {
    std::env::var_os(SPEC_WRAPPER_MODE_ENV_VAR).is_some()
}

pub fn run_rustc_wrapper_from_env() -> Result<ExitCode, RunRustcWrapperFromEnvError> {
    use RunRustcWrapperFromEnvError::*;
    let invocation = handle!(read_wrapper_invocation(), ReadWrapperInvocationFailed);
    handle!(collect_types_from_rustc_invocation(&invocation), CollectTypesFromRustcInvocationFailed);
    let rustc_status = handle!(run_rustc(&invocation), RunRustcFailed, rustc_path: PathBuf::from(&invocation.rustc_path));
    Ok(exit_code_from_status(rustc_status))
}

pub fn read_wrapper_invocation() -> Result<WrapperInvocation, ReadWrapperInvocationError> {
    use ReadWrapperInvocationError::*;
    let mut args = std::env::args_os();
    let _program_name = args.next();
    let rustc_path = handle_opt!(args.next(), MissingRustcPath);
    let selected_package = handle_opt!(
        std::env::var_os(SPEC_SELECTED_PACKAGE_ENV_VAR),
        MissingSelectedPackage,
        env_var: SPEC_SELECTED_PACKAGE_ENV_VAR.to_owned()
    );
    let output_dir = handle_opt!(
        std::env::var_os(SPEC_OUTPUT_DIR_ENV_VAR),
        MissingOutputDir,
        env_var: SPEC_OUTPUT_DIR_ENV_VAR.to_owned()
    );
    Ok(WrapperInvocation {
        rustc_path,
        rustc_args: args.collect(),
        selected_package: selected_package.to_string_lossy().into_owned(),
        output_dir: PathBuf::from(output_dir),
    })
}

pub fn collect_types_from_rustc_invocation(invocation: &WrapperInvocation) -> Result<(), CollectTypesFromRustcInvocationError> {
    use CollectTypesFromRustcInvocationError::*;
    let Some(current_package) = std::env::var_os("CARGO_PKG_NAME") else {
        return Ok(());
    };
    if std::env::var_os("CARGO_PRIMARY_PACKAGE").is_none() {
        return Ok(());
    }
    if current_package.to_string_lossy() != invocation.selected_package {
        return Ok(());
    }
    let Some((crate_name, input_path, active_cfgs)) = parse_rustc_invocation(&invocation.rustc_args) else {
        return Ok(());
    };
    if input_path
        .file_name()
        .and_then(|file_name| file_name.to_str())
        == Some("build.rs")
    {
        return Ok(());
    }
    let output_path = output_file_path(&invocation.output_dir, &crate_name, &input_path);
    let output_file = match OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&output_path)
    {
        Ok(output_file) => output_file,
        Err(source) if source.kind() == io::ErrorKind::AlreadyExists => return Ok(()),
        Err(source) => {
            return Err(CreateOutputFileFailed {
                source,
                output_path,
            });
        }
    };
    let collected_types = handle!(collect_types_from_file(&crate_name, &input_path, &active_cfgs), CollectTypesFromFileFailed, input_path);
    let mut output_writer = BufWriter::new(output_file);
    handle!(
        collected_types
            .iter()
            .try_for_each(|collected_type| writeln!(output_writer, "{collected_type}")),
        WriteOutputFileFailed,
        output_path: output_path.clone()
    );
    handle!(output_writer.flush(), FlushOutputFileFailed, output_path);
    Ok(())
}

pub fn parse_rustc_invocation(rustc_args: &impl AsRef<[OsString]>) -> Option<(String, PathBuf, ActiveCfgs)> {
    let rustc_args = rustc_args.as_ref();
    let crate_name = rustc_args
        .iter()
        .zip(rustc_args.iter().skip(1))
        .find_map(|(arg, next)| (arg == "--crate-name").then(|| next.to_string_lossy().into_owned()));
    let input_path = rustc_args.iter().find_map(|arg| {
        let path = Path::new(arg);
        (path.extension().and_then(|extension| extension.to_str()) == Some("rs")).then(|| path.to_path_buf())
    });
    let (Some(crate_name), Some(input_path)) = (crate_name, input_path) else {
        return None;
    };
    let active_cfgs = rustc_args
        .iter()
        .zip(rustc_args.iter().skip(1))
        .filter(|(arg, _)| *arg == "--cfg")
        .fold(ActiveCfgs::default(), |mut active_cfgs, (_, value)| {
            insert_cfg_value(&mut active_cfgs, value);
            active_cfgs
        });
    Some((crate_name, input_path, active_cfgs))
}

pub fn insert_cfg_value(active_cfgs: &mut ActiveCfgs, value: &impl AsRef<std::ffi::OsStr>) {
    let value = value.as_ref().to_string_lossy().into_owned();
    if let Some((name, cfg_value)) = value.split_once('=') {
        active_cfgs
            .pairs
            .insert((name.to_owned(), trim_cfg_value(cfg_value).to_owned()));
    } else {
        active_cfgs.names.insert(value);
    }
}

pub fn trim_cfg_value(value: &str) -> &str {
    value
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .unwrap_or(value)
}

pub fn output_file_path(output_dir: &impl AsRef<Path>, crate_name: &str, input_path: &impl AsRef<Path>) -> PathBuf {
    let mut hasher = DefaultHasher::new();
    crate_name.hash(&mut hasher);
    input_path.as_ref().hash(&mut hasher);
    output_dir
        .as_ref()
        .join(format!("{:016x}.txt", hasher.finish()))
}

pub fn run_rustc(invocation: &WrapperInvocation) -> io::Result<ExitStatus> {
    ProcessCommand::new(&invocation.rustc_path)
        .args(invocation.rustc_args.iter())
        .status()
}

pub fn exit_code_from_status(status: ExitStatus) -> ExitCode {
    if status.success() {
        return ExitCode::SUCCESS;
    }
    match status.code().and_then(|code| u8::try_from(code).ok()) {
        Some(code) => ExitCode::from(code),
        None => ExitCode::FAILURE,
    }
}

#[derive(Error, Debug)]
pub enum RunRustcWrapperFromEnvError {
    #[error("failed to read wrapper invocation")]
    ReadWrapperInvocationFailed { source: ReadWrapperInvocationError },
    #[error("failed to collect types from rustc invocation")]
    CollectTypesFromRustcInvocationFailed { source: Box<CollectTypesFromRustcInvocationError> },
    #[error("failed to run rustc at '{rustc_path}'")]
    RunRustcFailed { source: io::Error, rustc_path: PathBuf },
}

#[derive(Error, Debug)]
pub enum ReadWrapperInvocationError {
    #[error("missing rustc path in wrapper invocation")]
    MissingRustcPath,
    #[error("missing selected package environment variable '{env_var}'")]
    MissingSelectedPackage { env_var: String },
    #[error("missing output directory environment variable '{env_var}'")]
    MissingOutputDir { env_var: String },
}

#[derive(Error, Debug)]
pub enum CollectTypesFromRustcInvocationError {
    #[error("failed to create wrapper output file '{output_path}'")]
    CreateOutputFileFailed { source: io::Error, output_path: PathBuf },
    #[error("failed to collect types for input path '{input_path}'")]
    CollectTypesFromFileFailed { source: Box<CollectTypesFromFileError>, input_path: PathBuf },
    #[error("failed to write wrapper output file '{output_path}'")]
    WriteOutputFileFailed { source: io::Error, output_path: PathBuf },
    #[error("failed to flush wrapper output file '{output_path}'")]
    FlushOutputFileFailed { source: io::Error, output_path: PathBuf },
}
