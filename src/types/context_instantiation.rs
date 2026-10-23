use crate::{AlphaEq, Exp, Scope, Substitution, Sym, Typ};
use std::ptr::NonNull;

pub struct ContextInstantiation<'a> {
    pub scope: &'a Scope,
    pub substitution: Substitution,
}

impl<'a> ContextInstantiation<'a> {
    pub fn new(scope: &'a Scope, substitution: Substitution) -> Self {
        Self {
            scope,
            substitution,
        }
    }

    pub fn exp(&mut self, exp: &Exp) -> Exp {
        use Exp::*;
        match exp {
            Sol(sym) => {
                let key = NonNull::from(sym.as_ref());
                if let Some(value) = self.substitution.get(&key) {
                    return value.clone();
                }
                if self.scope.is_registered(sym) {
                    return exp.clone();
                }
                let typ = self.typ(sym.typ());
                let value = if typ.alpha_eq(sym.typ()) { exp.clone() } else { Sol(Sym::new_rc(sym.nym().clone(), typ)) };
                self.substitution.insert(key, value.clone());
                value
            }
            App(fun, arg, typ) => App(Box::new(self.exp(fun)), Box::new(self.exp(arg)), Box::new(self.typ(typ))),
        }
    }

    pub fn typ(&mut self, typ: &Typ) -> Typ {
        use Typ::*;
        match typ {
            Top => Top,
            One(exp) => One(self.exp(exp)),
            Fun(param, body) => {
                let key = NonNull::from(param.as_ref());
                let param_new = Sym::new_rc(param.nym().clone(), self.typ(param.typ()));
                let previous = self.substitution.insert(key, Exp::sol(&param_new));
                let body = self.typ(body);
                match previous {
                    Some(previous) => {
                        self.substitution.insert(key, previous);
                    }
                    None => {
                        self.substitution.remove(&key);
                    }
                }
                Fun(param_new, Box::new(body))
            }
        }
    }
}
