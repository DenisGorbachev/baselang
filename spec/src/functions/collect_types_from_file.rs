use crate::{ActiveCfgs, CollectedType, TypeKind};
use errgonomic::{handle, handle_opt};
use std::collections::BTreeSet;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use syn::punctuated::Punctuated;
use syn::{Attribute, Expr, Item, ItemMod, Lit, Meta, Token};
use thiserror::Error;

pub fn collect_types_from_file(crate_name: &impl AsRef<str>, input_path: &impl AsRef<Path>, active_cfgs: &ActiveCfgs) -> Result<BTreeSet<CollectedType>, CollectTypesFromFileError> {
    use CollectTypesFromFileError::*;
    let input_path = handle!(
        input_path.as_ref().canonicalize(),
        CanonicalizeModuleFileFailed,
        module_file: input_path.as_ref().to_path_buf()
    );
    let mut visited_paths = BTreeSet::new();
    collect_types_from_module_file(crate_name.as_ref(), &input_path, &[], active_cfgs, &mut visited_paths)
}

pub fn collect_types_from_module_file(crate_name: &str, module_file: &Path, module_path: &[String], active_cfgs: &ActiveCfgs, visited_paths: &mut BTreeSet<PathBuf>) -> Result<BTreeSet<CollectedType>, CollectTypesFromFileError> {
    use CollectTypesFromFileError::*;
    let module_file = handle!(
        module_file.canonicalize(),
        CanonicalizeModuleFileFailed,
        module_file: module_file.to_path_buf()
    );
    if !visited_paths.insert(module_file.clone()) {
        return Ok(BTreeSet::new());
    }
    let file_contents = handle!(read_to_string(&module_file), ReadModuleFileFailed, module_file: module_file.clone());
    let file = handle!(syn::parse_file(&file_contents), ParseModuleFileFailed, module_file: module_file.clone());
    collect_types_from_items(crate_name, &module_file, module_path, file.items, active_cfgs, visited_paths)
}

pub fn collect_types_from_items(crate_name: &str, module_file: &Path, module_path: &[String], items: Vec<Item>, active_cfgs: &ActiveCfgs, visited_paths: &mut BTreeSet<PathBuf>) -> Result<BTreeSet<CollectedType>, CollectTypesFromFileError> {
    use CollectTypesFromFileError::*;
    items
        .into_iter()
        .try_fold(BTreeSet::new(), |mut collected_types, item| {
            let item_types_result = match item {
                Item::Struct(item_struct) => {
                    if handle!(
                        are_cfg_attributes_enabled(&item_struct.attrs, active_cfgs),
                        EvaluateCfgAttributesFailed,
                        module_file: module_file.to_path_buf()
                    ) {
                        collected_types.insert(CollectedType {
                            kind: TypeKind::Struct,
                            path: item_path(crate_name, module_path, &item_struct.ident.to_string()),
                        });
                    }
                    return Ok(collected_types);
                }
                Item::Enum(item_enum) => {
                    if handle!(
                        are_cfg_attributes_enabled(&item_enum.attrs, active_cfgs),
                        EvaluateCfgAttributesFailed,
                        module_file: module_file.to_path_buf()
                    ) {
                        collected_types.insert(CollectedType {
                            kind: TypeKind::Enum,
                            path: item_path(crate_name, module_path, &item_enum.ident.to_string()),
                        });
                    }
                    return Ok(collected_types);
                }
                Item::Type(item_type) => {
                    if handle!(
                        are_cfg_attributes_enabled(&item_type.attrs, active_cfgs),
                        EvaluateCfgAttributesFailed,
                        module_file: module_file.to_path_buf()
                    ) {
                        collected_types.insert(CollectedType {
                            kind: TypeKind::Type,
                            path: item_path(crate_name, module_path, &item_type.ident.to_string()),
                        });
                    }
                    return Ok(collected_types);
                }
                Item::Union(item_union) => {
                    if handle!(
                        are_cfg_attributes_enabled(&item_union.attrs, active_cfgs),
                        EvaluateCfgAttributesFailed,
                        module_file: module_file.to_path_buf()
                    ) {
                        collected_types.insert(CollectedType {
                            kind: TypeKind::Union,
                            path: item_path(crate_name, module_path, &item_union.ident.to_string()),
                        });
                    }
                    return Ok(collected_types);
                }
                Item::Mod(item_mod) => {
                    if !handle!(
                        are_cfg_attributes_enabled(&item_mod.attrs, active_cfgs),
                        EvaluateCfgAttributesFailed,
                        module_file: module_file.to_path_buf()
                    ) {
                        return Ok(collected_types);
                    }
                    collect_types_from_module_item(crate_name, module_file, module_path, item_mod, active_cfgs, visited_paths)
                }
                _ => Ok(BTreeSet::new()),
            };
            let item_types = match item_types_result {
                Ok(item_types) => item_types,
                Err(source) => return Err(source),
            };
            collected_types.extend(item_types);
            Ok(collected_types)
        })
}

pub fn collect_types_from_module_item(crate_name: &str, module_file: &Path, module_path: &[String], item_mod: ItemMod, active_cfgs: &ActiveCfgs, visited_paths: &mut BTreeSet<PathBuf>) -> Result<BTreeSet<CollectedType>, CollectTypesFromFileError> {
    use CollectTypesFromFileError::*;
    let module_name = item_mod.ident.to_string();
    let child_module_path = module_path
        .iter()
        .cloned()
        .chain(core::iter::once(module_name.clone()))
        .collect::<Vec<_>>();
    match item_mod.content {
        Some((_, items)) => collect_types_from_items(crate_name, module_file, &child_module_path, items, active_cfgs, visited_paths),
        None => {
            let child_module_file = handle!(
                resolve_child_module_path(module_file, &item_mod),
                ResolveChildModulePathFailed,
                parent_file: module_file.to_path_buf(),
                module_name
            );
            collect_types_from_module_file(crate_name, &child_module_file, &child_module_path, active_cfgs, visited_paths)
        }
    }
}

pub fn item_path(crate_name: &str, module_path: &[String], item_name: &str) -> String {
    let path = module_path
        .iter()
        .fold(crate_name.to_owned(), |path, segment| format!("{path}::{segment}"));
    format!("{path}::{item_name}")
}

pub fn are_cfg_attributes_enabled(attributes: &[Attribute], active_cfgs: &ActiveCfgs) -> Result<bool, EvaluateCfgAttributesError> {
    attributes
        .iter()
        .filter(|attribute| attribute.path().is_ident("cfg"))
        .map(|attribute| evaluate_cfg_attribute(attribute, active_cfgs))
        .try_fold(true, |enabled, attribute_enabled| attribute_enabled.map(|attribute_enabled| enabled && attribute_enabled))
}

pub fn evaluate_cfg_attribute(attribute: &Attribute, active_cfgs: &ActiveCfgs) -> Result<bool, EvaluateCfgAttributesError> {
    use EvaluateCfgAttributesError::*;
    match &attribute.meta {
        Meta::List(list) => {
            let meta = handle!(list.parse_args::<Meta>(), ParseCfgExpressionFailed);
            evaluate_cfg_meta(&meta, active_cfgs)
        }
        _ => Ok(true),
    }
}

pub fn evaluate_cfg_meta(meta: &Meta, active_cfgs: &ActiveCfgs) -> Result<bool, EvaluateCfgAttributesError> {
    use EvaluateCfgAttributesError::*;
    match meta {
        Meta::Path(path) => Ok(path
            .get_ident()
            .map(|ident| active_cfgs.names.contains(&ident.to_string()))
            .unwrap_or(false)),
        Meta::NameValue(name_value) => {
            let Some(name) = name_value.path.get_ident().map(|ident| ident.to_string()) else {
                return Ok(false);
            };
            match &name_value.value {
                Expr::Lit(expr_lit) => match &expr_lit.lit {
                    Lit::Str(value) => Ok(active_cfgs.pairs.contains(&(name, value.value()))),
                    _ => Err(UnsupportedCfgLiteralFailed {
                        name,
                    }),
                },
                _ => Err(UnsupportedCfgLiteralFailed {
                    name,
                }),
            }
        }
        Meta::List(list) => {
            let nested = handle!(list.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated), ParseCfgExpressionFailed);
            if list.path.is_ident("all") {
                return nested
                    .into_iter()
                    .map(|meta| evaluate_cfg_meta(&meta, active_cfgs))
                    .try_fold(true, |enabled, item_enabled| item_enabled.map(|item_enabled| enabled && item_enabled));
            }
            if list.path.is_ident("any") {
                return nested
                    .into_iter()
                    .map(|meta| evaluate_cfg_meta(&meta, active_cfgs))
                    .try_fold(false, |enabled, item_enabled| item_enabled.map(|item_enabled| enabled || item_enabled));
            }
            if list.path.is_ident("not") {
                let mut nested = nested.into_iter();
                let nested_meta = handle_opt!(nested.next(), InvalidNotExpressionFailed, len: 0_usize);
                if nested.next().is_some() {
                    return Err(InvalidNotExpressionFailed {
                        len: 2_usize,
                    });
                }
                return evaluate_cfg_meta(&nested_meta, active_cfgs).map(|enabled| !enabled);
            }
            Ok(false)
        }
    }
}

#[allow(clippy::question_mark)]
pub fn resolve_child_module_path(parent_file: &Path, item_mod: &ItemMod) -> Result<PathBuf, ResolveChildModulePathError> {
    use ResolveChildModulePathError::*;
    let module_name = item_mod.ident.to_string();
    let parent_dir = handle_opt!(
        parent_file.parent(),
        ParentDirectoryMissing,
        parent_file: parent_file.to_path_buf()
    );
    let path_attribute_path = match path_attribute_path(parent_dir, parent_file, item_mod) {
        Ok(path_attribute_path) => path_attribute_path,
        Err(source) => return Err(source),
    };
    if let Some(path_attribute_path) = path_attribute_path {
        return Ok(path_attribute_path);
    }
    let file_name = handle_opt!(
        parent_file.file_name().and_then(|file_name| file_name.to_str()),
        ParentFileNameInvalid,
        parent_file: parent_file.to_path_buf()
    );
    let module_dir = if matches!(file_name, "lib.rs" | "main.rs" | "mod.rs") {
        parent_dir.to_path_buf()
    } else {
        let stem = handle_opt!(
            parent_file.file_stem().and_then(|stem| stem.to_str()),
            ParentFileStemInvalid,
            parent_file: parent_file.to_path_buf()
        );
        parent_dir.join(stem)
    };
    let file_candidate = module_dir.join(format!("{module_name}.rs"));
    if file_candidate.is_file() {
        return Ok(file_candidate);
    }
    let mod_candidate = module_dir.join(&module_name).join("mod.rs");
    if mod_candidate.is_file() {
        return Ok(mod_candidate);
    }
    Err(ModuleFileNotFound {
        parent_file: parent_file.to_path_buf(),
        module_name,
    })
}

pub fn path_attribute_path(parent_dir: &Path, parent_file: &Path, item_mod: &ItemMod) -> Result<Option<PathBuf>, ResolveChildModulePathError> {
    use ResolveChildModulePathError::*;
    let module_name = item_mod.ident.to_string();
    let path_attribute = item_mod
        .attrs
        .iter()
        .find(|attribute| attribute.path().is_ident("path"));
    let Some(path_attribute) = path_attribute else {
        return Ok(None);
    };
    match &path_attribute.meta {
        Meta::NameValue(name_value) => match &name_value.value {
            Expr::Lit(expr_lit) => match &expr_lit.lit {
                Lit::Str(path) => Ok(Some(parent_dir.join(path.value()))),
                _ => Err(PathAttributeValueInvalid {
                    parent_file: parent_file.to_path_buf(),
                    module_name,
                }),
            },
            _ => Err(PathAttributeValueInvalid {
                parent_file: parent_file.to_path_buf(),
                module_name,
            }),
        },
        _ => Err(PathAttributeValueInvalid {
            parent_file: parent_file.to_path_buf(),
            module_name,
        }),
    }
}

#[derive(Error, Debug)]
pub enum CollectTypesFromFileError {
    #[error("failed to canonicalize module file '{module_file}'")]
    CanonicalizeModuleFileFailed { source: std::io::Error, module_file: PathBuf },
    #[error("failed to read module file '{module_file}'")]
    ReadModuleFileFailed { source: std::io::Error, module_file: PathBuf },
    #[error("failed to parse module file '{module_file}'")]
    ParseModuleFileFailed { source: syn::Error, module_file: PathBuf },
    #[error("failed to evaluate cfg attributes in module file '{module_file}'")]
    EvaluateCfgAttributesFailed { source: EvaluateCfgAttributesError, module_file: PathBuf },
    #[error("failed to resolve child module '{module_name}' from '{parent_file}'")]
    ResolveChildModulePathFailed { source: ResolveChildModulePathError, parent_file: PathBuf, module_name: String },
}

#[derive(Error, Debug)]
pub enum EvaluateCfgAttributesError {
    #[error("failed to parse cfg expression")]
    ParseCfgExpressionFailed { source: syn::Error },
    #[error("cfg literal for '{name}' is unsupported")]
    UnsupportedCfgLiteralFailed { name: String },
    #[error("cfg not expression is invalid for '{len}' values")]
    InvalidNotExpressionFailed { len: usize },
}

#[derive(Error, Debug)]
pub enum ResolveChildModulePathError {
    #[error("parent directory missing for '{parent_file}'")]
    ParentDirectoryMissing { parent_file: PathBuf },
    #[error("parent file name is invalid for '{parent_file}'")]
    ParentFileNameInvalid { parent_file: PathBuf },
    #[error("parent file stem is invalid for '{parent_file}'")]
    ParentFileStemInvalid { parent_file: PathBuf },
    #[error("path attribute is invalid for module '{module_name}' from '{parent_file}'")]
    PathAttributeValueInvalid { parent_file: PathBuf, module_name: String },
    #[error("module file not found for module '{module_name}' from '{parent_file}'")]
    ModuleFileNotFound { parent_file: PathBuf, module_name: String },
}
