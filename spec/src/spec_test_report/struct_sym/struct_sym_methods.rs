use aist::{Adt, Ctx};
use facet::Facet;

#[derive(Facet, Debug)]
pub struct StructSymMethods {
    pub new: Result<StructSymMethodsNew, StructSymMethodsNewError>,
}

impl StructSymMethods {
    pub fn new(ctx: &Ctx<'_>, sym: Adt) -> Self {
        let new = StructSymMethodsNew::new(ctx, sym);
        Self {
            new,
        }
    }
}

impl StructSymMethods {}

mod struct_sym_methods_new;

pub use struct_sym_methods_new::*;
