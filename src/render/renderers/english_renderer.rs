use crate::{Exp, Form, NymEn, Render, Typ, Var};
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;
use std::borrow::Cow;

#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct EnglishRenderer {
    /// The name of the [`Typ::Top`]
    top: NymEn,
    form: Form,
}

impl EnglishRenderer {
    pub fn render_var_inner<'a>(&self, var: &'a Var, is_top_level: bool, with_type: bool, _wrapped: bool) -> Option<Cow<'a, str>> {
        let name = &var.nym().get(self.form)?.en.singular;
        if with_type {
            let typ = self.render_typ_inner(var.typ())?;
            let name_render = if is_top_level { name.to_capitalized() } else { name.to_canonical() };
            let article = Self::render_article(&typ);
            Some(Cow::Owned(format!("{name_render} is {article} {typ}")))
            // if wrapped { format!("({name} : {typ})") } else { format!("{name} : {typ}") }
        } else {
            Some(name.into())
        }
    }

    pub fn render_typ_inner(&self, _typ: &Typ) -> Option<Cow<str>> {
        // match typ {
        //     Typ::Top => self.top.clone(),
        //     Typ::One(exp) => self.render_exp_inner(exp, false, false),
        //     Typ::Fun(var, typ) => {
        //         format!("{var} -> {typ}",
        //             var = self.render_var_inner(var, false, true, true),
        //             typ = self.render_typ(typ).unwrap_or_default())
        //     }
        // }
        todo!()
    }

    pub fn render_exp_inner(&self, _exp: &Exp, _is_top_level: bool, _with_type: bool) -> Option<Cow<str>> {
        // match exp {
        //     Exp::Sol(var) => self.render_var_inner(var, is_top_level, with_type, true),
        //     Exp::App(fun, arg, typ) => {
        //         // We don't want to print the types of inner values, only the type of the current exp itself
        //         const WITH_TYPE_INNER: bool = false;
        //         let fun = self.render_exp_inner(fun, false, WITH_TYPE_INNER);
        //         let arg = self.render_exp_inner(arg, false, WITH_TYPE_INNER);
        //
        //         if with_type {
        //             let typ = self.render_typ(typ).unwrap_or_default();
        //             format!("({fun} {arg}) : {typ}")
        //         } else {
        //             format!("{fun} {arg}")
        //         }
        //     }
        // }
        todo!()
    }

    pub fn render_article(input: &str) -> &'static str {
        if input.starts_with(Self::VOWELS) { "an" } else { "a" }
    }

    pub const VOWELS: [char; 5] = ['a', 'e', 'u', 'o', 'i'];
}

impl Render for EnglishRenderer {
    fn render_var(&self, var: &Var) -> Option<String> {
        self.render_var_inner(var, true, true, false)
            .map(|cow| cow.into_owned())
    }

    fn render_typ(&self, typ: &Typ) -> Option<String> {
        self.render_typ_inner(typ).map(|cow| cow.into_owned())
    }

    fn render_exp(&self, exp: &Exp) -> Option<String> {
        self.render_exp_inner(exp, true, true)
            .map(|cow| cow.into_owned())
    }
}

impl Default for EnglishRenderer {
    fn default() -> Self {
        Self {
            top: NymEn::from("idea"),
            form: Default::default(),
        }
    }
}
