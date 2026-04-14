use rustc_hir::definitions::DefPath;
use rustc_middle::ty::TyCtxt;
use rustc_span::def_id::LocalDefId;

#[inline(always)]
pub fn to_def_path(tcx: TyCtxt<'_>) -> impl FnMut(LocalDefId) -> DefPath {
    move |id: LocalDefId| tcx.hir_def_path(id)
}

#[inline(always)]
pub fn to_def_path_str(tcx: TyCtxt<'_>) -> impl FnMut(LocalDefId) -> String {
    move |id: LocalDefId| tcx.def_path_str(id)
}

#[inline(always)]
pub fn to_def_paths(tcx: TyCtxt<'_>, iter: impl Iterator<Item = LocalDefId>) -> impl Iterator<Item = DefPath> {
    iter.map(to_def_path(tcx))
}
