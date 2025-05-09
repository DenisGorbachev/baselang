use crate::{Exp, Typ, Var};

pub trait Render {
    fn render_var(&self, var: &Var) -> String;

    fn render_typ(&self, typ: &Typ) -> String;

    fn render_exp(&self, exp: &Exp) -> String;
}
