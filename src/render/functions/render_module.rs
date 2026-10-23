use crate::{Render, SymsVec};

pub fn render_syms_vec(provider: &impl SymsVec, renderer: &impl Render) -> impl Iterator<Item = String> {
    provider
        .syms_vec()
        .into_iter()
        .flat_map(|x| renderer.render_sym(x.as_ref()))
}
