use rustc_hir::definitions::DefPath;
use rustc_middle::ty::TyCtxt;
use rustc_span::def_id::DefId;
use std::iter::empty;
use stub_macro::stub;

pub fn get_mutators_by_name(tcx: TyCtxt, _the_struct: &str, _the_field: &str) -> impl Iterator<Item = DefId> {
    get_mutators(tcx, stub!(), stub!())
}

pub fn get_mutators(_tcx: TyCtxt, _the_struct: DefId, _the_field: DefId) -> impl Iterator<Item = DefId> {
    // TODO
    empty()
}

#[inline(always)]
pub fn to_def_path(tcx: TyCtxt) -> impl FnMut(DefId) -> DefPath {
    move |id: DefId| tcx.def_path(id)
}

#[inline(always)]
pub fn to_def_paths(tcx: TyCtxt, iter: impl Iterator<Item = DefId>) -> impl Iterator<Item = DefPath> {
    iter.map(to_def_path(tcx))
}

#[cfg(test)]
mod tests {
    #[test]
    fn must_get_mutators() {
        let _user_rs = include_str!("../../tst/src/user.rs");
        // TODO: with_tcx doesn't work because user.rs depends on external crates
        // let mutators = with_tcx(user_rs, |tcx| {
        //     let mutators = get_mutators_by_name(tcx, "User", "name");
        //     to_def_paths(tcx, mutators).collect::<Vec<_>>()
        // });
        // TODO: `mutators` must include a path to `with_name` (from `Setters`)
    }
}
