use rustc_middle::ty::{self, FieldDef, TyCtxt};

pub struct Field<'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub field_def: &'tcx FieldDef,
}

impl<'tcx> Field<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>, field_def: &'tcx FieldDef) -> Self {
        Self {
            tcx,
            field_def,
        }
    }

    pub fn ty(&self) -> ty::Ty<'tcx> {
        self.tcx.type_of(self.field_def.did).instantiate_identity()
    }
}
