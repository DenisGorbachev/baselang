use crate::{Exp, Form, NymEn, Render, Typ, Var};
use Cow::*;
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;
use std::borrow::Cow;

#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct EnglishRenderer {
    /// The name of the [`Typ::Top`]
    #[new(into)]
    top: NymEn,
    form: Form,
    capitalize: bool,
}

impl Render for EnglishRenderer {
    fn render_var(&self, var: &Var) -> Option<String> {
        self.render_var_inner(var, true, true, true, false)
            .map(|cow| cow.into_owned())
    }

    fn render_typ(&self, typ: &Typ) -> Option<String> {
        self.render_typ_inner(typ).map(|cow| cow.into_owned())
    }

    fn render_exp(&self, exp: &Exp) -> Option<String> {
        self.render_exp_inner(exp, true, true, true)
            .map(|cow| cow.into_owned())
    }
}

impl EnglishRenderer {
    #[allow(clippy::collapsible_else_if)]
    pub fn render_var_inner<'a>(&self, var: &'a Var, is_top_level: bool, with_name: bool, with_type: bool, _wrapped: bool) -> Option<Cow<'a, str>> {
        if is_top_level {
            self.name_is_typ(var).map(Owned)
        } else {
            if with_name {
                if with_type { self.typ_called_name(var).map(Owned) } else { self.name_canonical(var) }
            } else {
                self.render_typ_inner_with_article(var.typ()).map(Owned)
            }
        }
    }

    pub fn name_canonical<'a>(&self, var: &'a Var) -> Option<Cow<'a, str>> {
        Some(var.nym().get(self.form)?.en.singular.to_canonical())
    }

    pub fn name_is_typ(&self, var: &Var) -> Option<String> {
        let name = &var.nym().get(self.form)?.en.singular;
        let name = if self.capitalize { name.to_capitalized() } else { name.to_canonical() };
        let typ = self.render_typ_inner_with_article(var.typ())?;
        Some(format!("{name} is {typ}"))
    }

    pub fn typ_called_name(&self, var: &Var) -> Option<String> {
        let name = &var.nym().get(self.form)?.en.singular;
        let typ = self.render_typ_inner_with_article(var.typ())?;
        Some(format!("{typ} called \"{name}\""))
    }

    pub fn render_typ_inner<'a>(&self, typ: &'a Typ) -> Option<Cow<'a, str>> {
        // TODO: Is it possible to return just Cow instead of Option<Cow>?
        match typ {
            Typ::Top => Some(Owned(self.top.singular.to_string())),
            Typ::One(exp) => self.render_exp_inner(exp, false, true, false),
            Typ::Fun(var, typ) => {
                let var_render = self.render_var_inner(var, false, true, false, true)?;
                let typ_render = self.render_typ_inner_with_article(typ)?;
                Some(Owned(format!("program that takes `{var_render}` and gives `{typ_render}`")))
            }
        }
    }

    pub fn render_typ_inner_with_article(&self, typ: &Typ) -> Option<String> {
        let typ = self.render_typ_inner(typ)?;
        let article = Self::render_article(&typ);
        Some(format!("{article} {typ}"))
    }

    pub fn render_exp_inner<'a>(&self, exp: &'a Exp, _is_top_level: bool, with_name: bool, with_type: bool) -> Option<Cow<'a, str>> {
        match exp {
            Exp::Sol(var) => self.render_var_inner(var, false, with_name, false, true),
            Exp::App(fun, arg, typ) => {
                // We don't want to print the types of inner values, only the type of the current exp itself
                const WITH_TYPE_INNER: bool = false;
                let fun_render = self.render_exp_inner(fun, false, false, WITH_TYPE_INNER)?;
                let arg_render = self.render_exp_inner(arg, false, false, WITH_TYPE_INNER)?;

                if with_type {
                    let typ_render = self.render_typ_inner(typ)?;
                    Some(Owned(format!("({fun_render} {arg_render}) : {typ_render}")))
                } else {
                    Some(Owned(format!("{fun_render} {arg_render}")))
                }
            }
        }
    }

    pub fn render_article(input: &str) -> &'static str {
        if input.starts_with(Self::VOWELS) { "an" } else { "a" }
    }

    pub const VOWELS: [char; 5] = ['a', 'e', 'u', 'o', 'i'];

    pub fn short() -> Self {
        Self::default()
    }

    pub fn long() -> Self {
        Self {
            form: Form::Long,
            ..Self::default()
        }
    }
}

impl Default for EnglishRenderer {
    fn default() -> Self {
        Self {
            top: NymEn::from("idea"),
            form: Default::default(),
            capitalize: false,
        }
    }
}
