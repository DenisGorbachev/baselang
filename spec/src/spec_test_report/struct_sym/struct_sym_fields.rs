use aist::{Adt, get_mutators_by_name, to_def_path_str};
use errgonomic::{handle_bool, handle_opt};
use facet::Facet;
use rustc_middle::ty::TyKind;
use thiserror::Error;

#[derive(Facet, Debug)]
pub struct StructSymFields {
    pub typ: StructSymFieldsTyp,
}

impl StructSymFields {
    /// `sym` must be a struct
    pub fn new(sym: Adt) -> Result<Self, StructSymFieldsNewError> {
        use StructSymFieldsNewError::*;
        let nym_field = handle_opt!(sym.field("nym"), NymNotFound);
        let typ_field = handle_opt!(sym.field("typ"), TypNotFound);
        let field_count = sym.all_fields().count();
        handle_bool!(field_count != 2, FieldCountInvalid, field_count);
        let nym_type = nym_field.ty();
        let typ_type = typ_field.ty();
        handle_bool!(!matches!((nym_type.kind(), typ_type.kind()), (TyKind::Param(nym), TyKind::Param(typ)) if nym.index != typ.index), FieldTypesInvalid, nym_type: nym_type.to_string(), typ_type: typ_type.to_string());
        let typ = StructSymFieldsTyp::new(sym);
        Ok(Self {
            typ,
        })
    }
}

pub fn field_is_not_mutated_by_methods(adt: Adt, field_name: &str) -> Result<(), StructSymFieldsFieldIsNotMutatedByMethodsError> {
    use StructSymFieldsFieldIsNotMutatedByMethodsError::*;
    let _field = handle_opt!(adt.field(field_name), FieldNotFound, field_name);
    let methods = get_mutators_by_name(adt.tcx, "Sym", field_name)
        .map(to_def_path_str(adt.tcx))
        .collect::<Vec<_>>();
    handle_bool!(!methods.is_empty(), MutatorsFound, field_name, methods);
    Ok(())
}

#[derive(Error, Facet, Debug)]
#[repr(u8)]
pub enum StructSymFieldsNewError {
    #[error("field `nym` not found")]
    NymNotFound {},
    #[error("field `typ` not found")]
    TypNotFound {},
    #[error("expected 2 symbol fields, found {field_count}")]
    FieldCountInvalid { field_count: usize },
    #[error("symbol field types must be distinct type parameters, found '{nym_type}' and '{typ_type}'")]
    FieldTypesInvalid { nym_type: String, typ_type: String },
}

#[derive(Error, Facet, Debug)]
#[repr(u8)]
pub enum StructSymFieldsFieldIsNotMutatedByMethodsError {
    #[error("field `{field_name}` not found")]
    FieldNotFound { field_name: String },
    #[error("field `{field_name}` is mutated by methods: {methods:?}")]
    MutatorsFound { field_name: String, methods: Vec<String> },
}

mod struct_sym_fields_typ;

pub use struct_sym_fields_typ::*;
