use aist::Adt;
use facet::Facet;

#[derive(Facet, Debug)]
pub struct StructVarMethods {
    // new: StructVarMethodsNew
}

impl StructVarMethods {
    pub fn new(_var: Adt) -> Self {
        todo!()
    }
}

impl StructVarMethods {}

mod struct_var_methods_new;

pub use struct_var_methods_new::*;
