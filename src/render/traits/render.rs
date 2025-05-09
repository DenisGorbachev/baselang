use crate::{Exp, Typ, Var};

pub trait Render {
    fn render_var(&self, var: &Var) -> Option<String>;

    fn render_typ(&self, typ: &Typ) -> Option<String>;

    fn render_exp(&self, exp: &Exp) -> Option<String>;
}
