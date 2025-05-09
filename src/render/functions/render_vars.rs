use crate::{Render, Var};

pub fn render_vars<VarRef: AsRef<Var>>(vars: impl IntoIterator<Item = VarRef>, renderers: &[Box<dyn Render>]) -> impl Iterator<Item = impl Iterator<Item = Option<String>>> {
    vars.into_iter().map(|var| {
        renderers
            .iter()
            .map(move |renderer| renderer.render_var(var.as_ref()))
    })
}
