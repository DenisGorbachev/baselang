use crate::{Render, Renderer, SymsVec};

pub fn render_syms_vec(provider: &impl SymsVec, renderer: &impl Render) -> impl Iterator<Item = String> {
    provider
        .syms_vec()
        .into_iter()
        .flat_map(|x| renderer.render_sym(x.as_ref()))
        .chain(
            provider
                .rewrites_vec()
                .into_iter()
                .flat_map(|(sym, rule)| renderer.render_rewrite(sym, rule)),
        )
}

pub fn render_module<'a>(provider: &'a impl SymsVec, renderers: &'a [Renderer]) -> impl Iterator<Item = impl Iterator<Item = Option<String>> + 'a> + 'a {
    provider
        .syms_vec()
        .into_iter()
        .map(|sym| (sym, None))
        .chain(
            provider
                .rewrites_vec()
                .into_iter()
                .map(|(sym, rule)| (sym, Some(rule))),
        )
        .map(move |(sym, rule)| {
            renderers.iter().map(move |renderer| match rule {
                Some(rule) => renderer.render_rewrite(sym, rule),
                None => renderer.render_sym(sym),
            })
        })
}
