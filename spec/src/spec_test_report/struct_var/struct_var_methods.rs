use aist::{Adt, Ctx};
use facet::Facet;

#[derive(Facet, Debug)]
pub struct StructVarMethods {
    pub new: Result<StructVarMethodsNew, StructVarMethodsNewError>,
}

impl StructVarMethods {
    pub fn new(ctx: &Ctx<'_>, var: Adt) -> Self {
        let new = StructVarMethodsNew::new(ctx, var);
        Self {
            new,
        }
    }
}

impl StructVarMethods {}

mod struct_var_methods_new;

pub use struct_var_methods_new::*;
