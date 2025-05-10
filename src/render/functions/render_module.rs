use crate::{Render, VarsVec};

pub fn render_vars_vec(provider: &impl VarsVec, renderer: &impl Render) -> impl Iterator<Item = String> {
    provider
        .vars_vec()
        .into_iter()
        .flat_map(|x| renderer.render_var(x.as_ref()))
}
