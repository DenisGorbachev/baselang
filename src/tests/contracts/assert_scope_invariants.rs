use crate::{Rewrite, Scope, ScopeApplyManyError, ScopeConvertibleError, ScopeInferError, ScopeNormalizeTypError, Sym, SymRc, Typ};
use errgonomic::{ErrVec, handle, handle_iter};
use std::collections::{HashMap, HashSet};
use std::ptr::NonNull;
use thiserror::Error;

pub fn assert_scope_invariants(scope: &Scope) -> Result<(), AssertScopeInvariantsError> {
    use AssertScopeInvariantsError::*;
    let bindings = scope.bindings();
    assert!(
        bindings
            .iter()
            .all(|(name, sym)| name == sym.nym().identifier()),
        "binding keys equal symbol identifiers"
    );
    assert!(
        scope.rewrites().keys().all(|key| bindings
            .values()
            .any(|sym| *key == NonNull::from(sym.as_ref()))),
        "rewrite keys identify registered allocations"
    );
    scope.rewrites().values().for_each(|rules| {
        assert!(
            rules
                .windows(2)
                .all(|pair| matches!(pair, [left, right] if left.patterns.len() == right.patterns.len())),
            "rules of one symbol have equal arity"
        );
    });
    let results = bindings.values().flat_map(|sym| {
        scope
            .rewrites_of(sym)
            .iter()
            .map(move |rule| assert_rewrite_type(scope, sym, rule))
    });
    handle_iter!(results, AssertRewriteTypeFailed);
    let results = bindings
        .values()
        .filter(|sym| scope.rewrites_of(sym).is_empty())
        .map(|sym| scope.normalize_typ(sym.typ().last()).map(|typ| (sym, typ)));
    let terminals = handle_iter!(results, NormalizeTypFailed);
    let expected = terminals
        .into_iter()
        .fold(HashMap::<NonNull<Sym>, HashSet<NonNull<Sym>>>::new(), |mut expected, (sym, terminal)| {
            if let Typ::One(exp) = terminal {
                expected
                    .entry(NonNull::from(exp.head().as_ref()))
                    .or_default()
                    .insert(NonNull::from(sym.as_ref()));
            }
            expected
        });
    let actual = scope
        .constructors()
        .iter()
        .map(|(head, syms)| {
            let unique = syms
                .iter()
                .map(|sym| NonNull::from(sym.as_ref()))
                .collect::<HashSet<_>>();
            assert_eq!(unique.len(), syms.len(), "constructor index has no duplicate allocation");
            (*head, unique)
        })
        .collect::<HashMap<_, _>>();
    assert_eq!(actual, expected, "constructor index equals normalized terminal-head predicate");
    Ok(())
}

pub fn assert_rewrite_type(scope: &Scope, sym: &SymRc, rule: &Rewrite) -> Result<(), AssertRewriteTypeError> {
    use AssertRewriteTypeError::*;
    let lhs = handle!(scope.apply_many(sym, &rule.patterns), ApplyManyFailed);
    let body = handle!(scope.infer(&rule.body), InferFailed);
    let valid = handle!(scope.convertible(lhs.typ(), &body), ConvertibleFailed);
    assert!(valid, "rule body has the applied head's type");
    Ok(())
}

#[derive(Error, Debug)]
pub enum AssertScopeInvariantsError {
    #[error("failed to check rewrite result types")]
    AssertRewriteTypeFailed { source: ErrVec<AssertRewriteTypeError> },
    #[error("failed to normalize constructor result types")]
    NormalizeTypFailed { source: ErrVec<ScopeNormalizeTypError> },
}

#[derive(Error, Debug)]
pub enum AssertRewriteTypeError {
    #[error("failed to apply the rule head to its patterns")]
    ApplyManyFailed { source: ScopeApplyManyError },
    #[error("failed to infer the rule body type")]
    InferFailed { source: ScopeInferError },
    #[error("failed to compare rule result types")]
    ConvertibleFailed { source: ScopeConvertibleError },
}
