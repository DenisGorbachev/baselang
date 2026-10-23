use super::StructSymFieldsFieldIsNotMutatedByMethodsError;
use aist::{Adt, Field};
use errgonomic::{handle_bool, handle_opt};
use facet::Facet;
use rustc_middle::ty;
use rustc_span::def_id::DefId;
use rustc_span::sym;
use thiserror::Error;

#[derive(Facet, Debug)]
pub struct StructSymFieldsConstructors {
    pub has_type_option_vec_sym: Result<(), StructSymFieldsConstructorsHasTypeOptionVecSymError>,
    pub mutators: Result<(), StructSymFieldsFieldIsNotMutatedByMethodsError>,
}

impl StructSymFieldsConstructors {
    pub fn new(sym: Adt) -> Result<Self, StructSymFieldsConstructorsNewError> {
        use StructSymFieldsConstructorsNewError::*;
        let constructors_field = handle_opt!(sym.field("constructors"), NotFound);
        let has_type_option_vec_sym = Self::has_type_option_vec_sym(sym, constructors_field);
        let mutators = super::field_is_not_mutated_by_methods(sym, "constructors");
        Ok(Self {
            has_type_option_vec_sym,
            mutators,
        })
    }

    fn has_type_option_vec_sym(sym: Adt, constructors: Field) -> Result<(), StructSymFieldsConstructorsHasTypeOptionVecSymError> {
        use StructSymFieldsConstructorsHasTypeOptionVecSymError::*;
        handle_bool!(!is_option_vec_of_def_id(constructors, sym.did()), TypeInvalid);
        Ok(())
    }
}

#[derive(Error, Facet, Debug)]
#[repr(u8)]
pub enum StructSymFieldsConstructorsNewError {
    #[error("field `constructors` not found")]
    NotFound {},
}

#[derive(Error, Facet, Debug)]
#[repr(u8)]
pub enum StructSymFieldsConstructorsHasTypeOptionVecSymError {
    #[error("field `constructors` does not have type `Option<Vec<Sym>>`")]
    TypeInvalid {},
}

fn is_option_vec_of_def_id(field: Field, def_id: DefId) -> bool {
    let tcx = field.tcx;

    let ty::Adt(option_def, option_args) = field.ty().kind() else {
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
    matches!(inner_type.kind(), ty::Adt(sym_def, _) if sym_def.did() == def_id)
}
