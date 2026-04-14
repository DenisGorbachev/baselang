use crate::IntoSymbol;
use rustc_hir::def::DefKind;
use rustc_hir::def_id::LocalDefId;
use rustc_hir::intravisit::{Visitor, walk_expr};
use rustc_hir::{Expr, ExprKind};
use rustc_middle::ty::TyCtxt;
use rustc_middle::ty::TypeckResults;
use rustc_span::Symbol;

pub fn get_mutators_by_name<'tcx>(tcx: TyCtxt<'tcx>, item_name: impl IntoSymbol, field_name: impl IntoSymbol) -> impl Iterator<Item = LocalDefId> + 'tcx {
    let item_name = item_name.into_symbol();
    let field_name = field_name.into_symbol();
    tcx.iter_local_def_id()
        .filter(move |local_def_id| [DefKind::Struct, DefKind::Enum, DefKind::Union].contains(&tcx.def_kind(*local_def_id)) && tcx.item_name(*local_def_id) == item_name)
        .flat_map(move |struct_def_id| get_field_local_def_ids(tcx, struct_def_id, field_name).flat_map(move |field_def_id| get_mutators(tcx, struct_def_id, field_def_id)))
}

pub fn get_mutators(tcx: TyCtxt<'_>, the_struct: LocalDefId, the_field: LocalDefId) -> impl Iterator<Item = LocalDefId> + '_ {
    tcx.iter_local_def_id()
        .filter(move |candidate_def_id| matches!(tcx.def_kind(*candidate_def_id), DefKind::Fn | DefKind::AssocFn) && function_mutates_field(tcx, *candidate_def_id, the_struct, the_field))
}

fn get_field_local_def_ids<'tcx>(tcx: TyCtxt<'tcx>, ty_local_id: LocalDefId, the_field: Symbol) -> impl Iterator<Item = LocalDefId> + 'tcx {
    tcx.adt_def(ty_local_id)
        .all_fields()
        .filter(move |field| field.name == the_field)
        .filter_map(|field| field.did.as_local())
}

fn function_mutates_field(tcx: TyCtxt<'_>, function_def_id: LocalDefId, the_struct: LocalDefId, the_field: LocalDefId) -> bool {
    let Some(body) = tcx.hir_maybe_body_owned_by(function_def_id) else {
        return false;
    };
    let mut visitor = FieldMutationVisitor::new(tcx.typeck_body(body.id()), the_struct, tcx.item_name(the_field));
    visitor.visit_expr(body.value);
    visitor.found
}

struct FieldMutationVisitor<'tcx> {
    typeck_results: &'tcx TypeckResults<'tcx>,
    the_struct: LocalDefId,
    the_field_name: Symbol,
    found: bool,
}

impl<'tcx> FieldMutationVisitor<'tcx> {
    fn new(typeck_results: &'tcx TypeckResults<'tcx>, the_struct: LocalDefId, the_field_name: Symbol) -> Self {
        Self {
            typeck_results,
            the_struct,
            the_field_name,
            found: false,
        }
    }

    fn is_target_field_access(&self, expr: &Expr<'tcx>) -> bool {
        match expr.kind {
            ExprKind::Field(base, ident) => ident.name == self.the_field_name && self.is_target_struct_expr(base),
            _ => false,
        }
    }

    fn is_target_struct_expr(&self, expr: &Expr<'tcx>) -> bool {
        self.typeck_results
            .expr_ty_adjusted(expr)
            .peel_refs()
            .ty_adt_def()
            .and_then(|adt| adt.did().as_local())
            .map(|adt| adt == self.the_struct)
            .unwrap_or(false)
    }
}

impl<'tcx> Visitor<'tcx> for FieldMutationVisitor<'tcx> {
    fn visit_expr(&mut self, expr: &'tcx Expr<'tcx>) {
        match expr.kind {
            ExprKind::Assign(lhs, _, _) | ExprKind::AssignOp(_, lhs, _) => {
                if self.is_target_field_access(lhs) {
                    self.found = true;
                }
            }
            _ => {}
        }

        if !self.found {
            walk_expr(self, expr);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{get_mutators_by_name, to_def_path_str, with_tcx};
    use errgonomic::handle_bool;
    use thiserror::Error;

    #[test]
    fn must_get_mutators() -> Result<(), MustGetMutatorsError> {
        use MustGetMutatorsError::*;
        let mutators = with_tcx(expanded_user_rs_from_setters(), |tcx| {
            get_mutators_by_name(tcx, "User", "name")
                .map(to_def_path_str(tcx))
                .collect::<Vec<_>>()
        });
        // must not include User::new and User::clone
        handle_bool!(mutators != [String::from("User::with_name")], UnexpectedMutators, mutators);
        Ok(())
    }

    fn expanded_user_rs_from_setters() -> &'static str {
        r#"
#![crate_type = "lib"]

#[derive(Clone)]
pub struct User {
    name: String,
}

impl User {
    pub fn new(name: String) -> Self {
        Self {
            name,
        }
    }

    pub fn with_name(mut self, value: String) -> Self {
        self.name = value;
        self
    }
}
"#
    }

    #[derive(Error, Debug)]
    enum MustGetMutatorsError {
        #[error("unexpected mutators: {mutators:?}")]
        UnexpectedMutators { mutators: Vec<String> },
    }
}
