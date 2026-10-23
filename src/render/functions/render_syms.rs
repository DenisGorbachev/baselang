use crate::{Render, Renderer, Sym};

pub fn render_syms<SymRef: AsRef<Sym>>(syms: impl IntoIterator<Item = SymRef>, renderers: &[Renderer]) -> impl Iterator<Item = impl Iterator<Item = Option<String>>> {
    syms.into_iter().map(|sym| {
        renderers
            .iter()
            .map(move |renderer| renderer.render_sym(sym.as_ref()))
    })
}
