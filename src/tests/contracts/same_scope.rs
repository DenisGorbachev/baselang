use crate::{Scope, same_exp};
use std::rc::Rc;

pub fn same_scope(left: &Scope, right: &Scope) -> bool {
    left.bindings().len() == right.bindings().len()
        && left.bindings().iter().all(|(name, sym)| {
            right
                .bindings()
                .get(name)
                .is_some_and(|other| Rc::ptr_eq(sym, other))
        })
        && left.rewrites().len() == right.rewrites().len()
        && left.rewrites().iter().all(|(head, rules)| {
            right.rewrites().get(head).is_some_and(|other| {
                rules.len() == other.len()
                    && rules.iter().zip(other).all(|(left, right)| {
                        left.patterns.len() == right.patterns.len()
                            && left
                                .patterns
                                .iter()
                                .zip(&right.patterns)
                                .all(|(left, right)| same_exp(left, right))
                            && same_exp(&left.body, &right.body)
                    })
            })
        })
        && left.constructors().len() == right.constructors().len()
        && left.constructors().iter().all(|(head, syms)| {
            right.constructors().get(head).is_some_and(|other| {
                syms.len() == other.len()
                    && syms
                        .iter()
                        .zip(other)
                        .all(|(left, right)| Rc::ptr_eq(left, right))
            })
        })
}
