use crate::var_struct_must_have_field_constructors_of_option_vec;
use aist::Adt;
use facet::Facet;

#[derive(Facet, Debug)]
#[repr(u8)]
pub enum StructVarFields {
    StructVarFieldsWithConstructors { constructors: Result<StructVarFieldsConstructorsOptionVec, StructVarFieldsConstructorsOptionVecNewError> },
    StructVarFieldsWithoutConstructors {},
}

impl StructVarFields {
    /// `var` must be a struct
    pub fn new(var: Adt) -> Self {
        if var_struct_must_have_field_constructors_of_option_vec() == Some(true) {
            let constructors = StructVarFieldsConstructorsOptionVec::new(var);
            Self::StructVarFieldsWithConstructors {
                constructors,
            }
        } else {
            Self::StructVarFieldsWithoutConstructors {}
        }
    }
}

mod struct_var_fields_constructors_option_vec;

pub use struct_var_fields_constructors_option_vec::*;
