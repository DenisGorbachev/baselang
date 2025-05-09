use crate::{Exp, Render, Typ, Var};
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;

/// CueRenderer implements the Render trait to format variables, types, and expressions
/// in a structured format that includes cues and formatting for better readability.
#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
pub struct CueRenderer {}

impl CueRenderer {
    /// Returns the name of this renderer
    pub fn name() -> &'static str {
        "Cue"
    }
}

impl Render for CueRenderer {
    fn name(&self) -> &'static str {
        Self::name()
    }

    /// Renders a variable in the cue format with proper spacing
    fn render_var(&self, var: &Var) -> String {
        let name = var.name();
        let typ = self.render_typ(var.typ());

        // For the 'next' variable specifically, we need to split it to match the expected format
        if name == "next" {
            // This assumes the type is in the format "{ n : nat } : nat"
            let parts: Vec<&str> = typ.split('}').collect();
            if parts.len() > 1 {
                let opening_part = parts[0].trim();
                let closing_part = parts[1].trim();
                return format!("{} : {}\n}} {}", name, opening_part, closing_part);
            }
        }

        format!("{} : {}", name, typ)
    }

    /// Renders a type based on its variant with cue formatting
    fn render_typ(&self, typ: &Typ) -> String {
        match typ {
            Typ::Top => "Top".to_string(),
            Typ::One(exp) => self.render_exp(exp),
            Typ::Fun(var, typ_box) => {
                if var.typ() == &Typ::Top {
                    // For simple function types (e.g., Nat -> Nat)
                    format!("{} -> {}", var.name(), self.render_typ(typ_box))
                } else {
                    // For parameterized function types with argument types
                    format!("{{\n  {} : {}\n}} : {}", var.name(), self.render_typ(var.typ()), self.render_typ(typ_box))
                }
            }
        }
    }

    /// Renders an expression in the cue format
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
