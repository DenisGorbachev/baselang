use crate::{Module, Render};

pub fn render_module(module: &impl Module, renderer: &impl Render) -> impl Iterator<Item = String> {
    module
        .vars_refs()
        .into_iter()
        .flat_map(|x| renderer.render_var(x.as_ref()))
}
