use aist::{Adt, Ctx};
use errgonomic::{handle_bool, handle_opt};
use facet::Facet;
use rustc_hir::def_id::{DefId, LocalDefId};
use rustc_span::sym;
use thiserror::Error;

#[derive(Facet, Debug)]
pub struct StructVarMethodsNew {
    pub is_constructor: Result<(), StructVarMethodsNewIsConstructorError>,
    pub has_same_named_args_as_fields: Result<(), StructVarMethodsNewHasSameNamedArgsAsFieldsError>,
}

impl StructVarMethodsNew {
    pub fn new(ctx: &Ctx<'_>, var: Adt) -> Result<Self, StructVarMethodsNewError> {
        use StructVarMethodsNewError::*;
        let new_function = handle_opt!(
            ctx.inherent_fn_def_ids_by_symbol(var.did(), sym::new)
                .next(),
            NotFound
        );
        let is_constructor = Self::is_constructor(ctx, var, new_function);
        let has_same_named_args_as_fields = Self::has_same_named_args_as_fields(var, new_function);
        Ok(Self {
            is_constructor,
            has_same_named_args_as_fields,
        })
    }

    fn is_constructor(ctx: &Ctx<'_>, var: Adt, new_function_def_id: DefId) -> Result<(), StructVarMethodsNewIsConstructorError> {
        use StructVarMethodsNewIsConstructorError::*;
        let new_function = var.tcx.associated_item(new_function_def_id);
        let signature = new_function.signature(var.tcx);
        handle_bool!(!ctx.is_constructor(new_function_def_id), SignatureInvalid, signature);
        Ok(())
    }

    fn has_same_named_args_as_fields(var: Adt, new_function_def_id: DefId) -> Result<(), StructVarMethodsNewHasSameNamedArgsAsFieldsError> {
        use StructVarMethodsNewHasSameNamedArgsAsFieldsError::*;
        let mut field_names = var
            .all_fields()
            .map(|field| field.name.to_string())
            .collect::<Vec<_>>();
        let mut param_names = get_param_names(var, new_function_def_id);

        field_names.sort();
        param_names.sort();

        let has_same_named_args_as_fields = field_names.len() == param_names.len()
            && field_names
                .iter()
                .zip(param_names.iter())
                .all(|(field_name, param_name)| param_name.as_ref() == Some(field_name));
        handle_bool!(!has_same_named_args_as_fields, NamesInvalid, field_names, param_names);
        Ok(())
    }
}

#[derive(Error, Facet, Debug)]
#[repr(u8)]
pub enum StructVarMethodsNewError {
    #[error("associated function `new` not found")]
    NotFound {},
}

#[derive(Error, Facet, Debug)]
#[repr(u8)]
pub enum StructVarMethodsNewIsConstructorError {
    #[error("associated function `new` is not a constructor")]
    SignatureInvalid { signature: String },
}

#[derive(Error, Facet, Debug)]
#[repr(u8)]
pub enum StructVarMethodsNewHasSameNamedArgsAsFieldsError {
    #[error("associated function `new` does not accept the same named args as `Var` fields")]
    NamesInvalid { field_names: Vec<String>, param_names: Vec<Option<String>> },
}

fn get_param_names(var: Adt, fn_new_def_id: DefId) -> Vec<Option<String>> {
    let Some(local_def_id) = fn_new_def_id.as_local() else {
        return vec![None];
    };
    get_local_param_names(var, local_def_id)
}

fn get_local_param_names(var: Adt, local_def_id: LocalDefId) -> Vec<Option<String>> {
    let Some(body_id) = var.tcx.hir_node_by_def_id(local_def_id).body_id() else {
        return vec![None];
    };
    var.tcx
        .hir_body_param_idents(body_id)
        .map(|ident| ident.map(|ident| ident.name.to_string()))
        .collect()
}
