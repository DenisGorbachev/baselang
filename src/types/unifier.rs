use crate::{Exp, PlainRenderer, Scope, ScopeNormalizeError, ScopeNormalizeTypError, Substitution, Sym, SymRc, Typ};
use errgonomic::{handle, handle_bool};
use std::collections::HashSet;
use std::ptr::NonNull;
use std::rc::Rc;
use thiserror::Error;

pub struct Unifier<'a> {
    pub scope: &'a Scope,
    pub flexible: HashSet<NonNull<Sym>>,
    pub substitution: Substitution,
}

impl<'a> Unifier<'a> {
    pub fn new(scope: &'a Scope, flexible: impl IntoIterator<Item = SymRc>) -> Self {
        Self {
            scope,
            flexible: flexible
                .into_iter()
                .map(|sym| NonNull::from(sym.as_ref()))
                .collect(),
            substitution: Substitution::new(),
        }
    }

    pub fn resolve(&self, exp: &Exp) -> Exp {
        let mut result = exp.clone();
        loop {
            let next = result.instantiate(&self.substitution);
            if next.same_term(&result) {
                return next;
            }
            result = next;
        }
    }

    pub fn is_rigid(&self, sym: &SymRc) -> bool {
        self.scope.is_registered(sym) && self.scope.rewrites_of(sym).is_empty()
    }

    pub fn occurs_rigidly(&self, sym: &SymRc, exp: &Exp) -> bool {
        use Exp::*;
        match exp {
            Sol(other) => Rc::ptr_eq(sym, other),
            App(fun, arg, _) => self.is_rigid(exp.head()) && (self.occurs_rigidly(sym, fun) || self.occurs_rigidly(sym, arg)),
        }
    }

    pub fn exp(&mut self, left: &Exp, right: &Exp) -> Result<bool, UnifierExpError> {
        use Exp::*;
        use UnifierExpError::*;
        let left = handle!(self.scope.normalize(&self.resolve(left)), NormalizeLeftFailed);
        let right = handle!(self.scope.normalize(&self.resolve(right)), NormalizeRightFailed);
        if left.same_term(&right) {
            return Ok(true);
        }
        let binding = match (&left, &right) {
            (Sol(sym), _) if self.flexible.contains(&NonNull::from(sym.as_ref())) => Some((sym, &right)),
            (_, Sol(sym)) if self.flexible.contains(&NonNull::from(sym.as_ref())) => Some((sym, &left)),
            _ => None,
        };
        if let Some((sym, value)) = binding {
            if value.contains_sym(sym) {
                handle_bool!(!self.occurs_rigidly(sym, value), EquationUndeterminedInvalid, left, right);
                return Ok(false);
            }
            if !handle!(self.typ(sym.typ(), value.typ()), TypFailed) {
                return Ok(false);
            }
            self.substitution
                .insert(NonNull::from(sym.as_ref()), self.resolve(value));
            return Ok(true);
        }
        handle_bool!(!self.is_rigid(left.head()) || !self.is_rigid(right.head()), EquationUndeterminedInvalid, left, right);
        match (&left, &right) {
            (App(left_fun, left_arg, _), App(right_fun, right_arg, _)) => {
                let fun = handle!(self.exp(left_fun, right_fun), ExpFunFailed);
                Ok(fun && handle!(self.exp(left_arg, right_arg), ExpArgFailed))
            }
            _ => Ok(false),
        }
    }

    pub fn typ(&mut self, left: &Typ, right: &Typ) -> Result<bool, UnifierTypError> {
        use Typ::*;
        use UnifierTypError::*;
        let left = handle!(
            self.scope
                .normalize_typ(&left.instantiate(&self.substitution)),
            NormalizeTypLeftFailed
        );
        let right = handle!(
            self.scope
                .normalize_typ(&right.instantiate(&self.substitution)),
            NormalizeTypRightFailed
        );
        match (left, right) {
            (Top, Top) => Ok(true),
            (One(left), One(right)) => Ok(handle!(self.exp(&left, &right), ExpFailed)),
            (Fun(left_param, left_body), Fun(right_param, right_body)) => {
                let domain = handle!(self.typ(left_param.typ(), right_param.typ()), TypDomainFailed);
                let right_body = right_body.replace(&right_param, &left_param);
                Ok(domain && handle!(self.typ(&left_body, &right_body), TypBodyFailed))
            }
            _ => Ok(false),
        }
    }
}

#[derive(Error, Debug)]
pub enum UnifierExpError {
    #[error("unification cannot establish whether '{left}' and '{right}' are equal or impossible to equate", left = PlainRenderer::default().render_exp_inner(left, false, false), right = PlainRenderer::default().render_exp_inner(right, false, false))]
    EquationUndeterminedInvalid { left: Box<Exp>, right: Box<Exp> },
    #[error("failed to normalize the left unification operand")]
    NormalizeLeftFailed { source: ScopeNormalizeError },
    #[error("failed to normalize the right unification operand")]
    NormalizeRightFailed { source: ScopeNormalizeError },
    #[error("failed to unify variable types")]
    TypFailed { source: Box<UnifierTypError> },
    #[error("failed to unify application functions")]
    ExpFunFailed { source: Box<Self> },
    #[error("failed to unify application arguments")]
    ExpArgFailed { source: Box<Self> },
}

#[derive(Error, Debug)]
pub enum UnifierTypError {
    #[error("failed to normalize the left type")]
    NormalizeTypLeftFailed { source: ScopeNormalizeTypError },
    #[error("failed to normalize the right type")]
    NormalizeTypRightFailed { source: ScopeNormalizeTypError },
    #[error("failed to unify type expressions")]
    ExpFailed { source: Box<UnifierExpError> },
    #[error("failed to unify function domains")]
    TypDomainFailed { source: Box<Self> },
    #[error("failed to unify function codomains")]
    TypBodyFailed { source: Box<Self> },
}
