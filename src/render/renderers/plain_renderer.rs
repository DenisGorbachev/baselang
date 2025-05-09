use crate::{Exp, Render, Typ, Var};
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;

#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
pub struct PlainRenderer {}

impl PlainRenderer {}

impl Render for PlainRenderer {
    fn render_var(&self, _var: &Var) -> String {
        todo!()
    }

    fn render_typ(&self, _typ: &Typ) -> String {
        todo!()
    }

    fn render_exp(&self, _exp: &Exp) -> String {
        todo!()
    }
}
