use crate::FnItem;
use derive_more::Deref;
use rustc_hir::Body;
use rustc_middle::ty::TyCtxt;

#[derive(Deref, Copy, Clone)]
pub struct Fn<'c> {
    #[deref]
    pub item: FnItem<'c>,
    pub tcx: TyCtxt<'c>,
}

impl<'c> Fn<'c> {
    pub fn new(item: FnItem<'c>, tcx: TyCtxt<'c>) -> Self {
        Self {
            item,
            tcx,
        }
    }

    pub fn body(&self) -> &'c Body<'c> {
        self.tcx.hir_body(self.item.body)
    }
}
