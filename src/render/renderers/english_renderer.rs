use crate::{Exp, Form, NymEn, Render, Typ, Var};
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;

#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct EnglishRenderer {
    /// The name of the [`Typ::Top`]
    top: NymEn,
    form: Form,
}

impl EnglishRenderer {
    pub fn render_var_inner(&self, _var: &Var, _is_top_level: bool, _with_type: bool, _wrapped: bool) -> String {
        // let name = &var.nym().get(self.form)?;
        // if with_type {
        //     let typ = self.render_typ(var.typ()).unwrap_or_default();
        //     if wrapped { format!("({name} : {typ})") } else { format!("{name} : {typ}") }
        // } else {
        //     name.to_string()
        // }
        todo!()
    }

    pub fn render_typ_inner(&self, _typ: &Typ) -> String {
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

    pub fn render_exp_inner(&self, _exp: &Exp, _is_top_level: bool, _with_type: bool) -> String {
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
}

impl Render for EnglishRenderer {
    fn render_var(&self, var: &Var) -> Option<String> {
        // We catch the panic from todo!() and convert it to None
        std::panic::catch_unwind(|| self.render_var_inner(var, true, true, false)).ok()
    }

    fn render_typ(&self, typ: &Typ) -> Option<String> {
        std::panic::catch_unwind(|| self.render_typ_inner(typ)).ok()
    }

    fn render_exp(&self, exp: &Exp) -> Option<String> {
        std::panic::catch_unwind(|| self.render_exp_inner(exp, true, true)).ok()
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
