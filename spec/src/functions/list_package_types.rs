use crate::{SPEC_OUTPUT_DIR_ENV_VAR, SPEC_SELECTED_PACKAGE_ENV_VAR, SPEC_WRAPPER_MODE_ENV_VAR};
use cargo_metadata::{Metadata, MetadataCommand};
use errgonomic::{ErrVec, handle, handle_iter, handle_opt, map_err};
use std::collections::BTreeSet;
use std::fs::{DirEntry, create_dir_all, read_dir, read_to_string};
use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;
use tempfile::TempDir;
use thiserror::Error;

pub fn list_package_types(manifest_path: &impl AsRef<Path>, package: Option<&str>) -> Result<String, ListPackageTypesError> {
    use ListPackageTypesError::*;
    let manifest_path = handle!(
        manifest_path.as_ref().canonicalize(),
        CanonicalizeManifestPathFailed,
        manifest_path: manifest_path.as_ref().to_path_buf()
    );
    let metadata = handle!(load_metadata(&manifest_path), LoadMetadataFailed, manifest_path);
    let selected_package = handle!(
        select_package_name(&metadata, &manifest_path, package),
        SelectPackageNameFailed,
        manifest_path,
        package: package.map(str::to_owned)
    );
    let temp_dir = handle!(TempDir::new(), CreateTempDirFailed);
    let output_dir = temp_dir.path().join("types");
    handle!(create_dir_all(&output_dir), CreateOutputDirFailed, output_dir);
    let target_dir = temp_dir.path().join("cargo-target");
    let rustc_wrapper = handle!(std::env::current_exe(), CurrentExeFailed);
    let mut cargo = ProcessCommand::new("cargo");
    cargo.arg("check");
    cargo.arg("--quiet");
    cargo.arg("--manifest-path");
    cargo.arg(&manifest_path);
    cargo.arg("--package");
    cargo.arg(&selected_package);
    cargo.arg("--target-dir");
    cargo.arg(&target_dir);
    cargo.env(SPEC_WRAPPER_MODE_ENV_VAR, "1");
    cargo.env(SPEC_SELECTED_PACKAGE_ENV_VAR, &selected_package);
    cargo.env(SPEC_OUTPUT_DIR_ENV_VAR, &output_dir);
    cargo.env("RUSTC_WRAPPER", rustc_wrapper);
    let cargo_status = handle!(
        cargo.status(),
        SpawnCargoCheckFailed,
        manifest_path,
        package: selected_package.clone()
    );
    if !cargo_status.success() {
        return Err(CargoCheckFailed {
            package: selected_package,
            code: cargo_status.code(),
        });
    }
    Ok(handle!(read_wrapper_outputs(&output_dir), ReadWrapperOutputsFailed, output_dir))
}

pub fn load_metadata(manifest_path: &impl AsRef<Path>) -> Result<Metadata, LoadMetadataError> {
    use LoadMetadataError::*;
    let manifest_path = manifest_path.as_ref();
    let mut metadata_command = MetadataCommand::new();
    metadata_command.no_deps();
    metadata_command.manifest_path(manifest_path);
    map_err!(
        metadata_command.exec(),
        ExecCargoMetadataFailed,
        manifest_path: manifest_path.to_path_buf()
    )
}

pub fn select_package_name(metadata: &Metadata, manifest_path: &impl AsRef<Path>, package: Option<&str>) -> Result<String, SelectPackageNameError> {
    use SelectPackageNameError::*;
    let manifest_path = manifest_path.as_ref();
    if let Some(package) = package {
        if let Some(selected_package) = metadata
            .packages
            .iter()
            .find(|candidate| candidate.name == package && metadata.workspace_members.contains(&candidate.id))
        {
            return Ok(selected_package.name.to_string());
        }
        return Err(PackageNotFound {
            manifest_path: manifest_path.to_path_buf(),
            package: package.to_owned(),
        });
    }

    if let Some(selected_package) = metadata
        .packages
        .iter()
        .find(|candidate| candidate.manifest_path.as_std_path() == manifest_path)
    {
        return Ok(selected_package.name.to_string());
    }

    if metadata.workspace_default_members.len() != 1 {
        return Err(DefaultPackageIsAmbiguous {
            manifest_path: manifest_path.to_path_buf(),
            len: metadata.workspace_default_members.len(),
        });
    }

    let default_member = handle_opt!(
        metadata.workspace_default_members.first(),
        DefaultPackageMissing,
        manifest_path: manifest_path.to_path_buf()
    );
    let selected_package = handle_opt!(
        metadata.packages.iter().find(|candidate| &candidate.id == default_member),
        DefaultPackageNotFound,
        manifest_path: manifest_path.to_path_buf()
    );
    Ok(selected_package.name.to_string())
}

pub fn read_wrapper_outputs(output_dir: &impl AsRef<Path>) -> Result<String, ReadWrapperOutputsError> {
    use ReadWrapperOutputsError::*;
    let output_dir = output_dir.as_ref();
    let output_entries = handle!(
        read_dir(output_dir),
        ReadOutputDirFailed,
        output_dir: output_dir.to_path_buf()
    );
    let output_entries = handle_iter!(
        output_entries,
        ReadOutputDirEntriesFailed,
        output_dir: output_dir.to_path_buf()
    );
    let output_lines = handle_iter!(
        output_entries.into_iter().map(read_wrapper_output_file_lines),
        ReadOutputFilesFailed,
        output_dir: output_dir.to_path_buf()
    );
    let mut type_lines = output_lines
        .into_iter()
        .flatten()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .fold(String::new(), |mut output, line| {
            if !output.is_empty() {
                output.push('\n');
            }
            output.push_str(&line);
            output
        });
    if !type_lines.is_empty() {
        type_lines.push('\n');
    }
    Ok(type_lines)
}

pub fn read_wrapper_output_file_lines(entry: DirEntry) -> Result<Vec<String>, ReadWrapperOutputFileError> {
    use ReadWrapperOutputFileError::*;
    let output_path = entry.path();
    let output = handle!(read_to_string(&output_path), ReadOutputFileFailed, output_path);
    Ok(output.lines().map(str::to_owned).collect())
}

#[derive(Error, Debug)]
pub enum ListPackageTypesError {
    #[error("failed to canonicalize manifest path '{manifest_path}'")]
    CanonicalizeManifestPathFailed { source: std::io::Error, manifest_path: PathBuf },
    #[error("failed to load cargo metadata for manifest path '{manifest_path}'")]
    LoadMetadataFailed { source: LoadMetadataError, manifest_path: PathBuf },
    #[error("failed to select a package for manifest path '{manifest_path}'")]
    SelectPackageNameFailed { source: SelectPackageNameError, manifest_path: PathBuf, package: Option<String> },
    #[error("failed to create a temporary directory")]
    CreateTempDirFailed { source: std::io::Error },
    #[error("failed to create output directory '{output_dir}'")]
    CreateOutputDirFailed { source: std::io::Error, output_dir: PathBuf },
    #[error("failed to locate the spec executable")]
    CurrentExeFailed { source: std::io::Error },
    #[error("failed to spawn cargo check for package '{package}'")]
    SpawnCargoCheckFailed { source: std::io::Error, manifest_path: PathBuf, package: String },
    #[error("cargo check failed for package '{package}'")]
    CargoCheckFailed { package: String, code: Option<i32> },
    #[error("failed to read wrapper output directory '{output_dir}'")]
    ReadWrapperOutputsFailed { source: ReadWrapperOutputsError, output_dir: PathBuf },
}

#[derive(Error, Debug)]
pub enum LoadMetadataError {
    #[error("failed to execute cargo metadata for manifest path '{manifest_path}'")]
    ExecCargoMetadataFailed { source: cargo_metadata::Error, manifest_path: PathBuf },
}

#[derive(Error, Debug)]
pub enum SelectPackageNameError {
    #[error("package '{package}' not found for manifest path '{manifest_path}'")]
    PackageNotFound { manifest_path: PathBuf, package: String },
    #[error("default package is ambiguous for manifest path '{manifest_path}'")]
    DefaultPackageIsAmbiguous { manifest_path: PathBuf, len: usize },
    #[error("default package is missing for manifest path '{manifest_path}'")]
    DefaultPackageMissing { manifest_path: PathBuf },
    #[error("default package metadata not found for manifest path '{manifest_path}'")]
    DefaultPackageNotFound { manifest_path: PathBuf },
}

#[derive(Error, Debug)]
pub enum ReadWrapperOutputsError {
    #[error("failed to read output directory '{output_dir}'")]
    ReadOutputDirFailed { source: std::io::Error, output_dir: PathBuf },
    #[error("failed to enumerate output directory '{output_dir}'")]
    ReadOutputDirEntriesFailed { source: ErrVec<std::io::Error>, output_dir: PathBuf },
    #[error("failed to read wrapper output files from '{output_dir}'")]
    ReadOutputFilesFailed { source: ErrVec<ReadWrapperOutputFileError>, output_dir: PathBuf },
}

#[derive(Error, Debug)]
pub enum ReadWrapperOutputFileError {
    #[error("failed to read wrapper output file '{output_path}'")]
    ReadOutputFileFailed { source: std::io::Error, output_path: PathBuf },
}
