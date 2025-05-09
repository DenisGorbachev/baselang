use crate::{Exp, Render, Typ, Var};
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;

/// PlainRenderer implements the Render trait to format variables, types, and expressions
/// in a plain text format suitable for standard output.
#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
pub struct PlainRenderer {}

impl PlainRenderer {
    /// Returns the name of this renderer
    pub fn name() -> &'static str {
        "Plain"
    }
}

impl Render for PlainRenderer {
    fn name(&self) -> &'static str {
        Self::name()
    }

    /// Renders a variable in the format "name : type"
    fn render_var(&self, var: &Var) -> String {
        format!("{} : {}", var.name(), self.render_typ(var.typ()))
    }

    /// Renders a type based on its variant (Top, One, Fun)
    fn render_typ(&self, typ: &Typ) -> String {
        match typ {
            Typ::Top => "top".to_string(),
            Typ::One(exp) => self.render_exp(exp),
            Typ::Fun(var, typ_box) => {
                let var_name = var.name();
                let var_typ = self.render_typ(var.typ());
                let result_typ = self.render_typ(typ_box);
                format!("({} : {}) -> {}", var_name, var_typ, result_typ)
            }
        }
    }

    /// Renders an expression based on its variant (Sol, App)
    fn render_exp(&self, exp: &Exp) -> String {
        match exp {
            Exp::Sol(var) => var.name().to_string(),
            Exp::App(fun, arg, _, _) => {
                let fun_str = self.render_exp(fun);
                let arg_str = self.render_exp(arg);
                format!("{} {}", fun_str, arg_str)
            }
        }
    }
}
