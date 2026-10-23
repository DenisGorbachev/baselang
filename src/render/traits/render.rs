use crate::{Def, Exp, Sym, Typ};

// TODO: Refactor to use a Writer
pub trait Render {
    fn render_sym(&self, sym: &Sym) -> Option<String>;

    fn render_def(&self, def: &Def) -> Option<String>;

    fn render_typ(&self, typ: &Typ) -> Option<String>;

    fn render_exp(&self, exp: &Exp) -> Option<String>;
}
