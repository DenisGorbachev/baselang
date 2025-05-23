use crate::{Exp, Render, Typ, Var};
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;
use std::borrow::Cow;

#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct PlainRenderer {
    /// The name of the [`Typ::Top`]
    #[new(into)]
    top: Cow<'static, str>,
}

impl PlainRenderer {
    pub fn render_var_inner(&self, var: &Var, _is_top_level: bool, with_type: bool, wrapped: bool) -> String {
        let name = &var.nym().short.en.singular;
        if with_type {
            let typ = self.render_typ_inner(var.typ());
            if wrapped { format!("({name} : {typ})") } else { format!("{name} : {typ}") }
        } else {
            name.to_string()
        }
    }

    pub fn render_typ_inner(&self, typ: &Typ) -> String {
        match typ {
            Typ::Top => self.top.clone().into_owned(),
            Typ::One(exp) => self.render_exp_inner(exp, false, false),
            Typ::Fun(var, typ) => {
                format!("{var} -> {typ}", var = self.render_var_inner(var, false, true, true), typ = self.render_typ_inner(typ))
            }
        }
    }

    pub fn render_exp_inner(&self, exp: &Exp, is_top_level: bool, with_type: bool) -> String {
        match exp {
            Exp::Sol(var) => self.render_var_inner(var, is_top_level, with_type, true),
            Exp::App(fun, arg, typ) => {
                // We don't want to print the types of inner values, only the type of the current exp itself
                const WITH_TYPE_INNER: bool = false;
                let fun = self.render_exp_inner(fun, false, WITH_TYPE_INNER);
                let arg = self.render_exp_inner(arg, false, WITH_TYPE_INNER);

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
    fn render_var(&self, var: &Var) -> Option<String> {
        Some(self.render_var_inner(var, true, true, false))
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
    use crate::{PlainRenderer, TestPrelude, must_print};

    must_print!(
        #[ignore]
        TestPrelude,
        PlainRenderer,
        "plain_renderer/test_prelude.plain.base"
    );
}
