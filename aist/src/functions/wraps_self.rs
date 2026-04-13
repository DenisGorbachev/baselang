use rustc_middle::ty::{Ty, TyKind};

pub fn wraps_self<'tcx>(candidate: Ty<'tcx>, self_ty: Ty<'tcx>) -> bool {
    candidate == self_ty
        || match candidate.kind() {
            TyKind::Adt(_, args) => args
                .iter()
                .filter_map(|arg| arg.as_type())
                .any(|arg_ty| wraps_self(arg_ty, self_ty)),
            TyKind::Alias(_, alias_ty) => alias_ty
                .args
                .iter()
                .filter_map(|arg| arg.as_type())
                .any(|arg_ty| wraps_self(arg_ty, self_ty)),
            _ => false,
        }
}
