use derive_more::Deref;
use rustc_middle::infer::canonical::ir::Interner;
use rustc_middle::infer::canonical::ir::inherent::SliceLike;
use rustc_middle::ty::{self, FieldDef, TyCtxt};

// Can't derive `Copy` or `Clone` because `FieldDef` doesn't derive them (likely an oversight in compiler code)
#[derive(Deref)]
pub struct Field<'c> {
    #[deref]
    pub def: &'c FieldDef,
    pub tcx: TyCtxt<'c>,
}

impl<'c> Field<'c> {
    pub fn new(def: &'c FieldDef, tcx: TyCtxt<'c>) -> Self {
        Self {
            def,
            tcx,
        }
    }

    pub fn ty(&self) -> ty::Ty<'c> {
        self.tcx.type_of(self.def.did).instantiate_identity()
    }

    pub fn ty_with_args<A>(&self, args: A) -> ty::Ty<'c>
    where
        A: SliceLike<Item = <TyCtxt<'c> as Interner>::GenericArg>,
    {
        self.tcx.type_of(self.def.did).instantiate(self.tcx, args)
    }
}
