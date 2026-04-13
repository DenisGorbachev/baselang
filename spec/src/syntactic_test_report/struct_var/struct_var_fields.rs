use aist::Adt;
use facet::Facet;
use spec::var_struct_must_have_field_constructors_of_option_vec;

#[derive(Facet, Debug)]
#[repr(u8)]
pub enum StructVarFields {
    StructVarFieldsWithConstructors { constructors: Result<StructVarFieldsConstructorsOptionVec, StructVarFieldsConstructorsOptionVecGatherError> },
    StructVarFieldsWithoutConstructors {},
}

impl StructVarFields {
    /// `var` must be a struct
    pub fn gather<'c>(var: Adt<'c>) -> Self {
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
