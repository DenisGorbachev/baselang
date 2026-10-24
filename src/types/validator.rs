use crate::{Coverage, CoverageCheckError, Exp, Positivity, PositivityCheckError, PositivityNewError, Rewrite, Scope, ScopeCheckRewriteError, ScopeConvertibleError, ScopeExpandPatternError, ScopeInferError, ScopeNormalizeError, ScopeNormalizeTypError, ScopeSaturateError, Substitution, Sym, SymRc, Termination, TerminationCheckError, Typ, Unifier, UnifierExpError, UnifierTypError};
use errgonomic::{ErrVec, handle, handle_bool, handle_iter};
use std::collections::HashSet;
use std::ptr::NonNull;
use thiserror::Error;

pub struct Validator<'a> {
    pub scope: &'a Scope,
}

impl<'a> Validator<'a> {
    pub fn new(scope: &'a Scope) -> Self {
        Self {
            scope,
        }
    }

    pub fn validate(&self) -> Result<(), ValidatorValidateError> {
        use ValidatorValidateError::*;
        handle_iter!(
            self.scope
                .bindings()
                .values()
                .map(|sym| self.declaration(sym)),
            DeclarationFailed
        );
        handle_iter!(self.scope.bindings().values().map(|sym| self.rules(sym)), RulesFailed);
        handle!(Termination::new(self.scope).check(), CheckTerminationFailed);
        let mut positivity = handle!(Positivity::new(self.scope), PositivityNewFailed);
        handle!(positivity.check(), CheckPositivityFailed);
        handle_iter!(
            self.scope
                .bindings()
                .values()
                .filter(|sym| !self.scope.rewrites_of(sym).is_empty())
                .map(|sym| Coverage::new(self.scope).check(sym)),
            CheckCoverageFailed
        );
        Ok(())
    }

    pub fn declaration(&self, sym: &SymRc) -> Result<(), ValidatorDeclarationError> {
        use ValidatorDeclarationError::*;
        handle!(self.typ(sym.typ(), &HashSet::new()), TypFailed);
        let typ = handle!(self.scope.normalize_typ(sym.typ()), NormalizeTypFailed);
        let sym_owned = sym.clone();
        handle_bool!(typ.contains_sym(sym), TypeCycleInvalid, sym: sym_owned);
        Ok(())
    }

    pub fn typ(&self, typ: &Typ, bound: &HashSet<NonNull<Sym>>) -> Result<(), ValidatorTypError> {
        use ValidatorTypError::*;
        match typ {
            Typ::Top => Ok(()),
            Typ::One(exp) => {
                handle!(self.exp(exp, bound), ExpFailed);
                Ok(())
            }
            Typ::Fun(param, body) => {
                handle!(self.typ(param.typ(), bound), TypParamFailed);
                let mut bound = bound.clone();
                bound.insert(NonNull::from(param.as_ref()));
                handle!(self.typ(body, &bound), TypBodyFailed);
                Ok(())
            }
        }
    }

    pub fn exp(&self, exp: &Exp, bound: &HashSet<NonNull<Sym>>) -> Result<(), ValidatorExpError> {
        use ValidatorExpError::*;
        if let Some(sym) = exp
            .symbols()
            .find(|sym| !self.scope.is_registered(sym) && !bound.contains(&NonNull::from(sym.as_ref())))
        {
            return Err(SymbolNotFound {
                sym: sym.clone(),
            });
        }
        if let Exp::App(fun, arg, _) = exp {
            handle!(self.exp(fun, bound), ExpFunFailed);
            handle!(self.exp(arg, bound), ExpArgFailed);
        }
        let inferred = handle!(self.scope.infer(exp), InferFailed);
        let valid = handle!(self.scope.convertible(&inferred, exp.typ()), ConvertibleFailed);
        handle_bool!(!valid, CachedTypeInvalid, inferred);
        Ok(())
    }

    pub fn rules(&self, sym: &SymRc) -> Result<(), ValidatorRulesError> {
        use ValidatorRulesError::*;
        let rules = self.scope.rewrites_of(sym);
        let expanded = handle_iter!(rules.iter().map(|rule| self.rule(sym, rule)), RuleFailed);
        handle_iter!(
            expanded
                .iter()
                .enumerate()
                .flat_map(|(left_index, left)| expanded
                    .iter()
                    .enumerate()
                    .filter(move |(right_index, _)| *right_index > left_index)
                    .map(move |(right_index, right)| self.overlap(sym, left, right, left_index, right_index))),
            OverlapFailed
        );
        Ok(())
    }

    pub fn rule(&self, sym: &SymRc, rule: &Rewrite) -> Result<Rewrite, ValidatorRuleError> {
        use ValidatorRuleError::*;
        let patterns = handle_iter!(
            rule.patterns
                .iter()
                .map(|pattern| self.scope.expand_pattern(pattern)),
            ExpandPatternFailed
        );
        let mut seen = HashSet::new();
        handle_iter!(
            patterns
                .iter()
                .map(|pattern| self.pattern(pattern, pattern.typ(), &mut seen)),
            PatternFailed
        );
        handle_iter!(patterns.iter().map(|pattern| self.exp(pattern, &seen)), ExpPatternFailed);
        handle_iter!(
            patterns
                .iter()
                .flat_map(Exp::symbols)
                .filter(|sym| !self.scope.is_registered(sym))
                .map(|sym| self.typ(sym.typ(), &seen)),
            TypFailed
        );
        handle!(self.exp(&rule.body, &seen), ExpFailed);
        let rule = Rewrite::new(patterns, rule.body.clone());
        handle!(self.scope.check_rewrite(sym, &rule), CheckRewriteFailed);
        Ok(rule)
    }

    pub fn pattern(&self, pattern: &Exp, expected: &Typ, seen: &mut HashSet<NonNull<Sym>>) -> Result<(), ValidatorPatternError> {
        use ValidatorPatternError::*;
        if let Exp::Sol(sym) = pattern
            && !self.scope.is_registered(sym)
        {
            let typ = handle!(self.scope.normalize_typ(sym.typ()), NormalizeTypFailed);
            let sym_owned = sym.clone();
            handle_bool!(matches!(typ, Typ::Fun(..)), FunctionVariableUnsupportedInvalid, sym: sym_owned);
            let sym_owned = sym.clone();
            handle_bool!(!seen.insert(NonNull::from(sym.as_ref())), VariableRepeatedInvalid, sym: sym_owned);
            return Ok(());
        }
        let (head, args) = pattern.spine();
        let head_owned = head.clone();
        handle_bool!(!self.scope.is_registered(head), PatternHeadInvalid, head: head_owned);
        let head_owned = head.clone();
        handle_bool!(!self.scope.rewrites_of(head).is_empty(), DefinedHeadInvalid, head: head_owned);
        let (saturated, params) = handle!(self.scope.saturate(head), SaturateFailed);
        let mut unifier = Unifier::new(self.scope, params.iter().cloned());
        let forced = args.len() == params.len() && handle!(unifier.typ(saturated.typ(), expected), TypFailed);
        let mut prefix = Substitution::new();
        let checks = args.iter().zip(&params).map(|(arg, param)| {
            use ValidatorPatternArgumentError::*;
            let key = NonNull::from(param.as_ref());
            let expected = param.typ().instantiate(&prefix);
            if forced && unifier.substitution.contains_key(&key) {
                let determined = unifier.resolve(&Exp::sol(param)).instantiate(&prefix);
                let actual = handle!(self.scope.normalize(arg), NormalizeActualFailed);
                let determined = handle!(self.scope.normalize(&determined), NormalizeDeterminedFailed);
                handle_bool!(!actual.same_term(&determined), ForcedArgumentInvalid, actual, determined);
            } else {
                handle!(self.pattern(arg, &expected, seen), PatternFailed);
            }
            prefix.insert(key, (*arg).clone());
            Ok(())
        });
        handle_iter!(checks, PatternArgumentFailed);
        Ok(())
    }

    pub fn overlap(&self, sym: &SymRc, left: &Rewrite, right: &Rewrite, left_index: usize, right_index: usize) -> Result<(), ValidatorOverlapError> {
        use ValidatorOverlapError::*;
        let right = right.freshened(self.scope);
        let mut unifier = Unifier::new(
            self.scope,
            left.variables(self.scope)
                .chain(right.variables(self.scope))
                .cloned(),
        );
        let overlap = handle!(
            left.patterns
                .iter()
                .zip(&right.patterns)
                .try_fold(true, |compatible, (left, right)| if compatible { unifier.exp(left, right) } else { Ok(false) }),
            ExpFailed
        );
        let sym = sym.clone();
        handle_bool!(overlap, RulesOverlapInvalid, sym, left_index, right_index);
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum ValidatorValidateError {
    #[error("failed to prepare strict positivity checking")]
    PositivityNewFailed { source: PositivityNewError },
    #[error("strict positivity could not be established")]
    CheckPositivityFailed { source: PositivityCheckError },
    #[error("invalid declarations")]
    DeclarationFailed { source: ErrVec<ValidatorDeclarationError> },
    #[error("invalid rewrite rules")]
    RulesFailed { source: ErrVec<ValidatorRulesError> },
    #[error("termination could not be established")]
    CheckTerminationFailed { source: TerminationCheckError },
    #[error("rewrite rules do not cover all arguments")]
    CheckCoverageFailed { source: ErrVec<CoverageCheckError> },
}

#[derive(Error, Debug)]
pub enum ValidatorDeclarationError {
    #[error("invalid declared type")]
    TypFailed { source: ValidatorTypError },
    #[error("failed to normalize the declared type")]
    NormalizeTypFailed { source: ScopeNormalizeTypError },
    #[error("symbol '{name}' occurs in its own normalized type", name = sym.nym().identifier())]
    TypeCycleInvalid { sym: SymRc },
}

#[derive(Error, Debug)]
pub enum ValidatorTypError {
    #[error("invalid type expression")]
    ExpFailed { source: Box<ValidatorExpError> },
    #[error("invalid parameter type")]
    TypParamFailed { source: Box<Self> },
    #[error("invalid codomain")]
    TypBodyFailed { source: Box<Self> },
}

#[derive(Error, Debug)]
pub enum ValidatorExpError {
    #[error("invalid function subexpression")]
    ExpFunFailed { source: Box<Self> },
    #[error("invalid argument subexpression")]
    ExpArgFailed { source: Box<Self> },
    #[error("unbound symbol '{name}'", name = sym.nym().identifier())]
    SymbolNotFound { sym: SymRc },
    #[error("failed to infer the expression type")]
    InferFailed { source: ScopeInferError },
    #[error("failed to compare the expression's cached and inferred types")]
    ConvertibleFailed { source: ScopeConvertibleError },
    #[error("the cached expression type is invalid")]
    CachedTypeInvalid { inferred: Box<Typ> },
}

#[derive(Error, Debug)]
pub enum ValidatorRulesError {
    #[error("invalid rule")]
    RuleFailed { source: ErrVec<ValidatorRuleError> },
    #[error("overlapping rules")]
    OverlapFailed { source: ErrVec<ValidatorOverlapError> },
}

#[derive(Error, Debug)]
pub enum ValidatorRuleError {
    #[error("invalid pattern expressions")]
    ExpPatternFailed { source: ErrVec<ValidatorExpError> },
    #[error("failed to expand argument patterns")]
    ExpandPatternFailed { source: ErrVec<ScopeExpandPatternError> },
    #[error("invalid argument patterns")]
    PatternFailed { source: ErrVec<ValidatorPatternError> },
    #[error("invalid pattern variable types")]
    TypFailed { source: ErrVec<ValidatorTypError> },
    #[error("invalid rule body")]
    ExpFailed { source: ValidatorExpError },
    #[error("rule does not preserve types")]
    CheckRewriteFailed { source: ScopeCheckRewriteError },
}

#[derive(Error, Debug)]
pub enum ValidatorPatternError {
    #[error("failed to normalize a pattern variable's type")]
    NormalizeTypFailed { source: ScopeNormalizeTypError },
    #[error("function-valued pattern variable '{name}' is unsupported", name = sym.nym().identifier())]
    FunctionVariableUnsupportedInvalid { sym: SymRc },
    #[error("pattern variable '{name}' occurs more than once outside forced arguments", name = sym.nym().identifier())]
    VariableRepeatedInvalid { sym: SymRc },
    #[error("pattern head '{name}' is not a registered symbol", name = head.nym().identifier())]
    PatternHeadInvalid { head: SymRc },
    #[error("defined symbol '{name}' remains in an argument pattern", name = head.nym().identifier())]
    DefinedHeadInvalid { head: SymRc },
    #[error("failed to instantiate the pattern constructor")]
    SaturateFailed { source: ScopeSaturateError },
    #[error("failed to infer forced constructor arguments")]
    TypFailed { source: UnifierTypError },
    #[error("invalid constructor arguments")]
    PatternArgumentFailed { source: ErrVec<ValidatorPatternArgumentError> },
}

#[derive(Error, Debug)]
pub enum ValidatorPatternArgumentError {
    #[error("failed to normalize the forced argument")]
    NormalizeActualFailed { source: ScopeNormalizeError },
    #[error("failed to normalize the argument determined by the result type")]
    NormalizeDeterminedFailed { source: ScopeNormalizeError },
    #[error("argument differs from the value determined by the constructor's result type")]
    ForcedArgumentInvalid { actual: Box<Exp>, determined: Box<Exp> },
    #[error("invalid argument pattern")]
    PatternFailed { source: Box<ValidatorPatternError> },
}

#[derive(Error, Debug)]
pub enum ValidatorOverlapError {
    #[error("failed to unify argument patterns")]
    ExpFailed { source: UnifierExpError },
    #[error("rules {left_index} and {right_index} for '{name}' overlap", name = sym.nym().identifier())]
    RulesOverlapInvalid { sym: SymRc, left_index: usize, right_index: usize },
}
