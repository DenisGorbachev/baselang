use crate::{Ctx, IntoSymbol};
use rustc_hir::def_id::DefId;

impl<'c> Ctx<'c> {
    pub fn inherent_fn_def_ids_by_symbol(&self, def_id: DefId, symbol: impl IntoSymbol) -> impl Iterator<Item = DefId> + '_ {
        let symbol = symbol.into_symbol();
        self.tcx
            .inherent_impls(def_id)
            .iter()
            .flat_map(move |impl_def_id| {
                self.tcx
                    .associated_items(*impl_def_id)
                    .filter_by_name_unhygienic(symbol)
            })
            .filter(|item| item.is_fn())
            .map(|item| item.def_id)
    }
}
