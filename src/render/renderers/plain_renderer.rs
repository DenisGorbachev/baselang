use crate::{Exp, Render, Rewrite, Sym, Typ};
use derive_more::{From, Into};
use derive_new::new;
use std::borrow::Cow;

#[derive(new, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct PlainRenderer {
    /// The name of the [`Typ::Top`]
    #[new(into)]
    pub top: Cow<'static, str>,
}

impl PlainRenderer {
    pub fn render_arguments(&self, arguments: &[Exp]) -> String {
        arguments
            .iter()
            .map(|arg| {
                let text = self.render_exp_inner(arg, false, false);
                if matches!(arg, Exp::App(..)) { format!("({text})") } else { text }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn render_sym_inner(&self, sym: &Sym, _is_top_level: bool, with_type: bool, wrapped: bool) -> String {
        let name = &sym.nym().short.en.singular;
        if with_type {
            let typ = self.render_typ_inner(sym.typ());
            if wrapped { format!("({name} : {typ})") } else { format!("{name} : {typ}") }
        } else {
            name.to_string()
        }
    }

    pub fn render_typ_inner(&self, typ: &Typ) -> String {
        match typ {
            Typ::Top => self.top.clone().into_owned(),
            Typ::One(exp) => self.render_exp_inner(exp, false, false),
            Typ::Fun(param, typ) => {
                format!("{sym} -> {typ}", sym = self.render_sym_inner(param, false, true, true), typ = self.render_typ_inner(typ))
            }
        }
    }

    pub fn render_exp_inner(&self, exp: &Exp, is_top_level: bool, with_type: bool) -> String {
        match exp {
            Exp::Sol(sym) => self.render_sym_inner(sym, is_top_level, with_type, true),
            Exp::App(fun, arg, typ) => {
                // We don't want to print the types of inner values, only the type of the current exp itself
                const WITH_TYPE_INNER: bool = false;
                let fun = self.render_exp_inner(fun, false, WITH_TYPE_INNER);
                let arg_render = self.render_exp_inner(arg, false, WITH_TYPE_INNER);
                let arg = if matches!(arg.as_ref(), Exp::App(..)) { format!("({arg_render})") } else { arg_render };

                if with_type {
                    let typ = self.render_typ_inner(typ);
                    format!("({fun} {arg}) : {typ}")
                } else {
                    format!("{fun} {arg}")
                }
            }
        }
    }

    pub fn idea() -> PlainRenderer {
        Self {
            top: "idea".into(),
        }
    }
}

impl Render for PlainRenderer {
    fn render_sym(&self, sym: &Sym) -> Option<String> {
        Some(self.render_sym_inner(sym, true, true, false))
    }

    fn render_rewrite(&self, sym: &Sym, rewrite: &Rewrite) -> Option<String> {
        let name = &sym.nym().short.en.singular;
        let lhs = rewrite
            .patterns
            .iter()
            .fold(name.to_string(), |mut text, pattern| {
                text.push(' ');
                let pattern_text = self.render_exp_inner(pattern, false, false);
                if matches!(pattern, Exp::App(..)) {
                    text.push('(');
                    text.push_str(&pattern_text);
                    text.push(')');
                } else {
                    text.push_str(&pattern_text);
                }
                text
            });
        let body = self.render_exp_inner(&rewrite.body, false, false);
        Some(format!("{lhs} => {body}"))
    }

    fn render_typ(&self, typ: &Typ) -> Option<String> {
        Some(self.render_typ_inner(typ))
    }

    fn render_exp(&self, exp: &Exp) -> Option<String> {
        Some(self.render_exp_inner(exp, true, true))
    }
}

impl Default for PlainRenderer {
    fn default() -> Self {
        Self {
            top: "top".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{PlainRenderer, TestPrelude, TestPreludeNewError, assert_eq_prints};
    use errgonomic::handle;
    use thiserror::Error;

    #[test]
    #[ignore]
    fn must_print() -> Result<(), MustPrintError> {
        use MustPrintError::*;
        let module = handle!(TestPrelude::new(), NewFailed);
        assert_eq_prints(&module, &PlainRenderer::default(), include_str!("plain_renderer/test_prelude.plain.base"));
        Ok(())
    }

    #[derive(Error, Debug)]
    pub enum MustPrintError {
        #[error("failed to construct the prelude")]
        NewFailed { source: TestPreludeNewError },
    }
}
