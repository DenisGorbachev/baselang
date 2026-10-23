use crate::{Call, Exp, Rewrite, Scope, ScopeExpandPatternError, ScopeNormalizeTypError, SymRc, Typ};
use errgonomic::{ErrVec, handle, handle_bool, handle_iter, handle_opt};
use thiserror::Error;

pub struct Termination<'a> {
    pub scope: &'a Scope,
}

impl<'a> Termination<'a> {
    pub fn new(scope: &'a Scope) -> Self {
        Self {
            scope,
        }
    }

    pub fn check(&self) -> Result<(), TerminationCheckError> {
        use TerminationCheckError::*;
        let calls = self.scope.bindings().values().flat_map(|sym| {
            self.scope
                .rewrites_of(sym)
                .iter()
                .map(move |rule| self.rule_calls(sym, rule))
        });
        let mut paths = handle_iter!(calls, RuleCallsFailed)
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
        let mut index = 0usize;
        while let Some(path) = paths.get(index).cloned() {
            handle_bool!(path.fails_descent(), DescentInvalid, path);
            let compositions = paths
                .iter()
                .flat_map(|other| [path.compose(other), other.compose(&path)])
                .flatten()
                .filter(|composition| !paths.iter().any(|existing| existing.same(composition)))
                .fold(Vec::<Call>::new(), |mut unique, composition| {
                    if !unique.iter().any(|existing| existing.same(&composition)) {
                        unique.push(composition);
                    }
                    unique
                });
            paths.extend(compositions);
            index = handle_opt!(index.checked_add(1), PathCountInvalid);
        }
        Ok(())
    }

    pub fn rule_calls(&self, sym: &SymRc, rule: &Rewrite) -> Result<Vec<Call>, TerminationRuleCallsError> {
        use TerminationRuleCallsError::*;
        if let Exp::Sol(value) = &rule.body
            && !self.scope.is_registered(value)
        {
            let typ = handle!(self.scope.normalize_typ(value.typ()), NormalizeTypFailed);
            let value = value.clone();
            handle_bool!(matches!(typ, Typ::Fun(..)), FunctionVariableReturnedInvalid, value);
        }
        let patterns = handle_iter!(
            rule.patterns
                .iter()
                .map(|pattern| self.scope.expand_pattern(pattern)),
            ExpandPatternFailed
        );
        Ok(handle!(self.calls(sym, &patterns, &rule.body), CallsFailed))
    }

    pub fn calls(&self, caller: &SymRc, patterns: &[Exp], exp: &Exp) -> Result<Vec<Call>, TerminationCallsError> {
        use TerminationCallsError::*;
        let (head, args) = exp.spine();
        let head_owned = head.clone();
        handle_bool!(!args.is_empty() && !self.scope.is_registered(head), HigherOrderCallInvalid, head: head_owned);
        let mut calls = handle_iter!(args.iter().map(|arg| self.calls(caller, patterns, arg)), CallsArgumentFailed)
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
        if let Some(rule) = self.scope.rewrites_of(head).first() {
            calls.push(Call::new(caller.clone(), head.clone(), patterns, &args, rule.patterns.len()));
        }
        Ok(calls)
    }
}

#[derive(Error, Debug)]
pub enum TerminationCheckError {
    #[error("failed to construct the call graph")]
    RuleCallsFailed { source: ErrVec<TerminationRuleCallsError> },
    #[error("a recursive cycle through '{name}' has no proven structural descent", name = path.from.nym().identifier())]
    DescentInvalid { path: Call },
    #[error("call graph path count exceeds addressable memory")]
    PathCountInvalid {},
}

#[derive(Error, Debug)]
pub enum TerminationRuleCallsError {
    #[error("returning function-valued pattern variable '{name}' requires termination evidence", name = value.nym().identifier())]
    FunctionVariableReturnedInvalid { value: SymRc },
    #[error("failed to normalize the returned pattern variable's type")]
    NormalizeTypFailed { source: ScopeNormalizeTypError },
    #[error("failed to expand recursive argument patterns")]
    ExpandPatternFailed { source: ErrVec<ScopeExpandPatternError> },
    #[error("failed to inspect rule calls")]
    CallsFailed { source: TerminationCallsError },
}

#[derive(Error, Debug)]
pub enum TerminationCallsError {
    #[error("calling pattern variable '{name}' requires termination evidence", name = head.nym().identifier())]
    HigherOrderCallInvalid { head: SymRc },
    #[error("failed to inspect calls in arguments")]
    CallsArgumentFailed { source: ErrVec<Self> },
}
