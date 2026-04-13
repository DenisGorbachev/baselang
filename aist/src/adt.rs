use crate::{Field, IntoSymbol};
use derive_more::Deref;
use rustc_middle::ty::{AdtDef, TyCtxt};

#[derive(Deref, Copy, Clone)]
pub struct Adt<'c> {
    #[deref]
    pub def: AdtDef<'c>,
    pub tcx: TyCtxt<'c>,
}

impl<'c> Adt<'c> {
    pub fn new(def: AdtDef<'c>, tcx: TyCtxt<'c>) -> Self {
        Self {
            def,
            tcx,
        }
    }

    pub fn field(&self, name: impl IntoSymbol) -> Option<Field<'c>> {
        let name = name.into_symbol();
        self.all_fields()
            .find(|def| def.name == name)
            .map(|def| Field::new(def, self.tcx))
    }
}
