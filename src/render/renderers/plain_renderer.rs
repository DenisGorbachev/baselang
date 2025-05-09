use crate::{Exp, Render, Typ, Var};
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;

#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
pub struct PlainRenderer {}

impl PlainRenderer {
    pub fn render_var_name<'a>(&self, var: &'a Var) -> &'a str {
        var.name()
    }

    pub fn render_var_with_type(&self, var: &Var) -> String {
        let name = self.render_var_name(var);
        let typ = self.render_typ(var.typ());
        format!("{name} : {typ}")
    }

    pub fn render_var_inner(&self, var: &Var, _is_top_level: bool, with_type: bool) -> String {
        let name = self.render_var_name(var);
        if with_type {
            let typ = self.render_typ(var.typ());
            format!("({name} : {typ})")
        } else {
            name.to_string()
        }
    }

    pub fn render_exp_inner(&self, exp: &Exp, is_top_level: bool, with_type: bool) -> String {
        match exp {
            Exp::Sol(var) => self.render_var_inner(var, is_top_level, with_type),
            Exp::App(fun, arg, _, typ) => {
                // We don't want to print the types of inner values, only the type of the current exp itself
                const WITH_TYPE_INNER: bool = false;
                let fun = self.render_exp_inner(fun, false, WITH_TYPE_INNER);
                let arg = self.render_exp_inner(arg, false, WITH_TYPE_INNER);

                if with_type {
                    let typ = self.render_typ(typ);
                    format!("({fun} {arg}) : {typ}")
                } else {
                    format!("{fun} {arg}")
                }
            }
        }
    }
}

impl Render for PlainRenderer {
    fn render_var(&self, var: &Var) -> String {
        self.render_var_with_type(var)
    }

    fn render_typ(&self, typ: &Typ) -> String {
        match typ {
            Typ::Top => "top".to_string(),
            Typ::One(exp) => self.render_exp_inner(exp, false, false),
            Typ::Fun(var, typ) => {
                format!("{var} -> {typ}", var = self.render_var_inner(var, false, true), typ = self.render_typ(typ))
            }
        }
    }

    fn render_exp(&self, exp: &Exp) -> String {
        self.render_exp_inner(exp, true, true)
    }
}
