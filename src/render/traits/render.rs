use crate::{Exp, Rewrite, Sym, Typ};

// TODO: Refactor to use a Writer
pub trait Render {
    fn render_sym(&self, sym: &Sym) -> Option<String>;

    fn render_rewrite(&self, sym: &Sym, rewrite: &Rewrite) -> Option<String>;

    fn render_typ(&self, typ: &Typ) -> Option<String>;

    fn render_exp(&self, exp: &Exp) -> Option<String>;
}
