#![cfg(test)]
#![allow(dead_code)]

use crate::Typ::Top;
use crate::{Var, VarRc};

pub fn nat() -> VarRc {
    Var::new_top_rc("Nat")
}

pub fn nat_constructors(nat: &VarRc) -> (VarRc, VarRc) {
    let n = Var::new("n", nat.clone());
    let next = Var::new_rc("Next", (n, nat.clone().into()));
    let zero = Var::new_rc("Zero", nat.clone());
    (next, zero)
}

pub fn list() -> VarRc {
    let t = Var::new_top("t");
    Var::new_rc("List", (t, Top))
}

#[test]
fn must_render_nat() {}
