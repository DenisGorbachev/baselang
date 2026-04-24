use crate::{Exp, Typ, VarRc};
use std::rc::Rc;

/// Semantic equality up to alpha-equivalence.
pub trait AlphaEq {
    fn alpha_eq(&self, other: &Self) -> bool;
}

pub fn alpha_eq_typ(left: &Typ, right: &Typ) -> bool {
    AlphaEqCtx::default().typ(left, right)
}

pub fn alpha_eq_exp(left: &Exp, right: &Exp) -> bool {
    AlphaEqCtx::default().exp(left, right)
}

#[derive(Default)]
struct AlphaEqCtx {
    bindings: Vec<(VarRc, VarRc)>,
}

impl AlphaEqCtx {
    fn typ(&mut self, left: &Typ, right: &Typ) -> bool {
        use Typ::*;
        match (left, right) {
            (Top, Top) => true,
            (One(left_exp), One(right_exp)) => self.exp(left_exp, right_exp),
            (Fun(left_param, left_typ), Fun(right_var, right_typ)) => {
                if !self.typ(left_param.typ(), right_var.typ()) {
                    return false;
                }
                self.bindings.push((left_param.clone(), right_var.clone()));
                let is_body_equal = self.typ(left_typ, right_typ);
                self.bindings.pop();
                is_body_equal
            }
            _ => false,
        }
    }

    fn exp(&mut self, left: &Exp, right: &Exp) -> bool {
        use Exp::*;
        match (left, right) {
            (Sol(left_var), Sol(right_var)) => self.same_var_occurrence(left_var, right_var),
            (App(left_fun, left_arg, left_typ), App(right_fun, right_arg, right_typ)) => self.exp(left_fun, right_fun) && self.exp(left_arg, right_arg) && self.typ(left_typ, right_typ),
            _ => false,
        }
    }

    fn same_var_occurrence(&self, left: &VarRc, right: &VarRc) -> bool {
        if let Some((_, mapped_right)) = self
            .bindings
            .iter()
            .rev()
            .find(|(mapped_left, _)| Rc::ptr_eq(mapped_left, left))
        {
            Rc::ptr_eq(mapped_right, right)
        } else if self
            .bindings
            .iter()
            .any(|(_, mapped_right)| Rc::ptr_eq(mapped_right, right))
        {
            false
        } else {
            Rc::ptr_eq(left, right)
        }
    }
}
