use aist::{Adt, get_mutators_by_name, to_def_path_str};
use errgonomic::{handle_bool, handle_opt};
use facet::Facet;
use thiserror::Error;

#[derive(Facet, Debug)]
pub struct StructSymFields {
    pub constructors: Result<StructSymFieldsConstructors, StructSymFieldsConstructorsNewError>,
    pub typ: StructSymFieldsTyp,
}

impl StructSymFields {
    /// `sym` must be a struct
    pub fn new(sym: Adt) -> Self {
        let constructors = StructSymFieldsConstructors::new(sym);
        let typ = StructSymFieldsTyp::new(sym);
        Self {
            constructors,
            typ,
        }
    }
}

fn field_is_not_mutated_by_methods(adt: Adt, field_name: &str) -> Result<(), StructSymFieldsFieldIsNotMutatedByMethodsError> {
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
pub enum StructSymFieldsFieldIsNotMutatedByMethodsError {
    #[error("field `{field_name}` not found")]
    FieldNotFound { field_name: String },
    #[error("field `{field_name}` is mutated by methods: {methods:?}")]
    MutatorsFound { field_name: String, methods: Vec<String> },
}

mod struct_sym_fields_constructors;

pub use struct_sym_fields_constructors::*;

mod struct_sym_fields_typ;

pub use struct_sym_fields_typ::*;
