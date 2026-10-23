use crate::{Call, Exp, Rewrite, Scope, Substitution, Sym, SymRc, Typ};
use errgonomic::{ErrVec, handle, handle_bool, partition_result};
use std::ptr::NonNull;
use std::rc::Rc;
use thiserror::Error;

pub struct Normalizer<'a> {
    pub scope: &'a Scope,
    pub zero_only: bool,
    pub calls: Vec<(SymRc, Vec<Exp>)>,
}

impl<'a> Normalizer<'a> {
    pub fn new(scope: &'a Scope, zero_only: bool) -> Self {
        Self {
            scope,
            zero_only,
            calls: Vec::new(),
        }
    }

    pub fn exp(&mut self, exp: &Exp) -> Result<Exp, NormalizerExpError> {
        use Exp::*;
        use NormalizerExpError::*;
        let exp = match exp {
            Sol(sym) => Sol(sym.clone()),
            App(fun, arg, typ) => {
                let fun = handle!(self.exp(fun), ExpFunFailed);
                let arg = handle!(self.exp(arg), ExpArgFailed);
                let typ = handle!(self.typ(typ), TypFailed);
                App(Box::new(fun), Box::new(arg), Box::new(typ))
            }
        };
        let result = handle!(self.reduce(&exp), ReduceFailed);
        Ok(result.unwrap_or(exp))
    }

    pub fn typ(&mut self, typ: &Typ) -> Result<Typ, NormalizerTypError> {
        use NormalizerTypError::*;
        use Typ::*;
        match typ {
            Top => Ok(Top),
            One(exp) => Ok(One(handle!(self.exp(exp), ExpFailed))),
            Fun(param, body) => {
                let param_typ = handle!(self.typ(param.typ()), TypParamFailed);
                let param_new = Sym::new_rc(param.nym().clone(), param_typ);
                let body = body.replace(param, &param_new);
                let body = handle!(self.typ(&body), TypBodyFailed);
                Ok(Fun(param_new, Box::new(body)))
            }
        }
    }

    pub fn reduce(&mut self, exp: &Exp) -> Result<Option<Exp>, NormalizerReduceError> {
        use NormalizerReduceError::*;
        let (head, args) = exp.spine();
        let scope = self.scope;
        let selected = scope
            .rewrites_of(head)
            .iter()
            .find_map(|rule| match self.match_rule(rule, &args) {
                Ok(Some(substitution)) => Some(Ok((rule, substitution))),
                Ok(None) => None,
                Err(error) => Some(Err(error)),
            });
        let Some(selected) = selected else { return Ok(None) };
        let (rule, substitution) = handle!(selected, MatchRuleFailed);
        let decreases = !self
            .calls
            .iter()
            .filter(|(sym, _)| Rc::ptr_eq(sym, head))
            .any(|(_, previous)| Call::new(head.clone(), head.clone(), previous, &args, args.len()).fails_descent());
        let head = head.clone();
        handle_bool!(!decreases, DescentInvalid, head, arguments: args.into_iter().cloned().collect::<Vec<_>>());
        let body = rule.body.instantiate(&substitution);
        self.calls
            .push((head.clone(), args.into_iter().cloned().collect()));
        let result = self.exp(&body);
        self.calls.pop();
        Ok(Some(handle!(result, ExpFailed)))
    }

    pub fn match_rule(&mut self, rule: &Rewrite, args: &[&Exp]) -> Result<Option<Substitution>, NormalizerMatchRuleError> {
        use NormalizerMatchRuleError::*;
        if (self.zero_only && !rule.patterns.is_empty()) || rule.patterns.len() != args.len() {
            return Ok(None);
        }
        let zero_only = self.zero_only;
        self.zero_only = true;
        let result = partition_result(rule.patterns.iter().map(|pattern| self.exp(pattern)));
        self.zero_only = zero_only;
        let patterns = handle!(result, ExpandPatternsFailed);
        let mut substitution = Substitution::new();
        Ok(patterns
            .iter()
            .zip(args)
            .all(|(pattern, arg)| self.matches(pattern, arg, &mut substitution))
            .then_some(substitution))
    }

    pub fn matches(&self, pattern: &Exp, arg: &Exp, substitution: &mut Substitution) -> bool {
        use Exp::*;
        match (pattern, arg) {
            (Sol(sym), _) if !self.scope.is_registered(sym) => {
                let key = NonNull::from(sym.as_ref());
                match substitution.get(&key) {
                    Some(previous) => previous.same_term(arg),
                    None => {
                        substitution.insert(key, arg.clone());
                        true
                    }
                }
            }
            (Sol(left), Sol(right)) => Rc::ptr_eq(left, right),
            (App(left_fun, left_arg, _), App(right_fun, right_arg, _)) => self.matches(left_fun, right_fun, substitution) && self.matches(left_arg, right_arg, substitution),
            _ => false,
        }
    }
}

#[derive(Error, Debug)]
pub enum NormalizerExpError {
    #[error("failed to normalize the function")]
    ExpFunFailed { source: Box<Self> },
    #[error("failed to normalize the argument")]
    ExpArgFailed { source: Box<Self> },
    #[error("failed to normalize the application type")]
    TypFailed { source: Box<NormalizerTypError> },
    #[error("failed to reduce the expression")]
    ReduceFailed { source: Box<NormalizerReduceError> },
}

#[derive(Error, Debug)]
pub enum NormalizerTypError {
    #[error("failed to normalize the type expression")]
    ExpFailed { source: Box<NormalizerExpError> },
    #[error("failed to normalize a parameter type")]
    TypParamFailed { source: Box<Self> },
    #[error("failed to normalize the result type")]
    TypBodyFailed { source: Box<Self> },
}

#[derive(Error, Debug)]
pub enum NormalizerReduceError {
    #[error("failed to match a rewrite rule")]
    MatchRuleFailed { source: NormalizerMatchRuleError },
    #[error("recursive reduction of '{name}' does not decrease its arguments", name = head.nym().identifier())]
    DescentInvalid { head: SymRc, arguments: Vec<Exp> },
    #[error("failed to normalize the substituted rule body")]
    ExpFailed { source: Box<NormalizerExpError> },
}

#[derive(Error, Debug)]
pub enum NormalizerMatchRuleError {
    #[error("failed to unfold definitions in argument patterns")]
    ExpandPatternsFailed { source: ErrVec<NormalizerExpError> },
}
