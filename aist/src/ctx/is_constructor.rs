use crate::{Ctx, wraps_self};
use rustc_hir::def::DefKind::{AssocFn, Impl};
use rustc_hir::def_id::DefId;

impl<'c> Ctx<'c> {
    pub fn is_constructor(&self, function_def_id: DefId) -> bool {
        let parent_def_id = self.parent(function_def_id);
        if self.def_kind(function_def_id) != AssocFn || !matches!(self.def_kind(parent_def_id), Impl { .. }) {
            return false;
        }

        let self_ty = self.type_of(parent_def_id).instantiate_identity();
        let output_ty = self
            .fn_sig(function_def_id)
            .instantiate_identity()
            .skip_binder()
            .output();

        wraps_self(output_ty, self_ty)
    }
}
