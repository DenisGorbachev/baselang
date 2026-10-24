use baselang::{AlphaEq, Exp};
use std::rc::Rc;

pub fn same_exp(left: &Exp, right: &Exp) -> bool {
    match (left, right) {
        (Exp::Sol(left), Exp::Sol(right)) => Rc::ptr_eq(left, right),
        (Exp::App(lf, la, lt), Exp::App(rf, ra, rt)) => same_exp(lf, rf) && same_exp(la, ra) && lt.alpha_eq(rt),
        _ => false,
    }
}
