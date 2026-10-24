use crate::Scope;
use crate::same_exp;
use aist_core::{iter_eq_by, maps_eq_by};
use std::rc::Rc;

pub fn same_scope(left: &Scope, right: &Scope) -> bool {
    maps_eq_by(left.bindings(), right.bindings(), Rc::ptr_eq) && maps_eq_by(left.rewrites(), right.rewrites(), |left, right| iter_eq_by(left, right, |left, right| iter_eq_by(&left.patterns, &right.patterns, same_exp) && same_exp(&left.body, &right.body))) && maps_eq_by(left.constructors(), right.constructors(), |left, right| iter_eq_by(left, right, Rc::ptr_eq))
}
