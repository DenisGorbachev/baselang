use crate::{ContextInstantiation, Exp, Normalizer, PlainRenderer, Rewrite, Scope, ScopeExpandPatternError, ScopeNormalizeTypError, ScopeParametersError, ScopeSaturateError, SymRc, Unifier, UnifierExpError, UnifierTypError};
use errgonomic::{ErrVec, handle, handle_iter};
use std::rc::Rc;
use thiserror::Error;

pub struct Coverage<'a> {
    pub scope: &'a Scope,
}

impl<'a> Coverage<'a> {
    pub fn new(scope: &'a Scope) -> Self {
        Self {
            scope,
        }
    }

    pub fn check(&self, sym: &SymRc) -> Result<(), CoverageCheckError> {
        use CoverageCheckError::*;
        let rules = self.scope.rewrites_of(sym);
        let arity = rules.first().map_or(0, |rule| rule.patterns.len());
        let params = handle!(self.scope.parameters(sym.typ()), ParametersFailed);
        let goals = params
            .into_iter()
            .take(arity)
            .map(Exp::Sol)
            .collect::<Vec<_>>();
        let expanded = handle_iter!(
            rules.iter().map(|rule| {
                use CoverageExpandRuleError::*;
                let patterns = handle_iter!(
                    rule.patterns
                        .iter()
                        .map(|pattern| self.scope.expand_pattern(pattern)),
                    ExpandPatternFailed
                );
                Ok(Rewrite::new(patterns, rule.body.clone()))
            }),
            ExpandRuleFailed
        );
        if let Some(arguments) = handle!(self.missing(&expanded, goals), MissingFailed) {
            return Err(CasesMissingInvalid {
                sym: sym.clone(),
                arguments,
            });
        }
        Ok(())
    }

    pub fn missing(&self, rules: &[Rewrite], goals: Vec<Exp>) -> Result<Option<Vec<Exp>>, CoverageMissingError> {
        use CoverageMissingError::*;
        let normalizer = Normalizer::new(self.scope, false);
        let covered = rules.iter().any(|rule| {
            let mut substitution = Default::default();
            rule.patterns
                .iter()
                .zip(&goals)
                .all(|(pattern, goal)| normalizer.matches(pattern, goal, &mut substitution))
        });
        if covered {
            return Ok(None);
        }
        let compatible = handle_iter!(rules.iter().map(|rule| self.compatible(rule, &goals)), CompatibleFailed);
        let rules = rules
            .iter()
            .zip(compatible)
            .filter(|(_, compatible)| *compatible)
            .map(|(rule, _)| rule.clone())
            .collect::<Vec<_>>();
        let split = rules.iter().find_map(|rule| {
            rule.patterns
                .iter()
                .zip(&goals)
                .find_map(|(pattern, goal)| self.split_symbol(pattern, goal))
        });
        let Some(split) = split else {
            let empty = handle_iter!(
                goals
                    .iter()
                    .flat_map(Exp::symbols)
                    .filter(|sym| !self.scope.is_registered(sym))
                    .map(|sym| self.uninhabited(sym)),
                UninhabitedFailed
            )
            .into_iter()
            .any(|empty| empty);
            return Ok((!empty).then_some(goals));
        };
        let typ = handle!(self.scope.normalize_typ(split.typ()), NormalizeTypFailed);
        let Some(head) = typ.head() else { return Ok(Some(goals)) };
        if !self.scope.is_registered(head) {
            return Ok(Some(goals));
        }
        let mut branches = self
            .scope
            .constructors_of(head)
            .iter()
            .map(|constructor| self.branch(&rules, &goals, &split, constructor));
        let result = branches.find_map(|result| match result {
            Ok(None) => None,
            result => Some(result),
        });
        match result {
            None => Ok(None),
            Some(result) => Ok(handle!(result, BranchFailed)),
        }
    }

    pub fn compatible(&self, rule: &Rewrite, goals: &[Exp]) -> Result<bool, CoverageCompatibleError> {
        use CoverageCompatibleError::*;
        let rule = rule.freshened(self.scope);
        let flexible = rule.variables(self.scope).cloned().chain(
            goals
                .iter()
                .flat_map(Exp::symbols)
                .filter(|sym| !self.scope.is_registered(sym))
                .cloned(),
        );
        let mut unifier = Unifier::new(self.scope, flexible);
        Ok(handle!(
            rule.patterns
                .iter()
                .zip(goals)
                .try_fold(true, |compatible, (pattern, goal)| if compatible { unifier.exp(pattern, goal) } else { Ok(false) }),
            ExpFailed
        ))
    }

    pub fn split_symbol(&self, pattern: &Exp, goal: &Exp) -> Option<SymRc> {
        if let Exp::Sol(sym) = pattern
            && !self.scope.is_registered(sym)
        {
            return None;
        }
        if let Exp::Sol(sym) = goal
            && !self.scope.is_registered(sym)
        {
            return Some(sym.clone());
        }
        let (pattern_head, pattern_args) = pattern.spine();
        let (goal_head, goal_args) = goal.spine();
        if Rc::ptr_eq(pattern_head, goal_head) {
            pattern_args
                .into_iter()
                .zip(goal_args)
                .find_map(|(pattern, goal)| self.split_symbol(pattern, goal))
        } else {
            None
        }
    }

    pub fn branch(&self, rules: &[Rewrite], goals: &[Exp], split: &SymRc, constructor: &SymRc) -> Result<Option<Vec<Exp>>, CoverageBranchError> {
        use CoverageBranchError::*;
        let (candidate, params) = handle!(self.scope.saturate(constructor), SaturateFailed);
        let flexible = params.into_iter().chain(
            goals
                .iter()
                .flat_map(Exp::symbols)
                .filter(|sym| !self.scope.is_registered(sym))
                .cloned(),
        );
        let mut unifier = Unifier::new(self.scope, flexible);
        if !handle!(unifier.typ(candidate.typ(), split.typ()), TypFailed) {
            return Ok(None);
        }
        if !handle!(unifier.exp(&Exp::sol(split), &candidate), ExpFailed) {
            return Ok(None);
        }
        let substitution = unifier
            .substitution
            .iter()
            .map(|(key, value)| (*key, unifier.resolve(value)))
            .collect();
        let mut context = ContextInstantiation::new(self.scope, substitution);
        let goals = goals.iter().map(|goal| context.exp(goal)).collect();
        Ok(handle!(self.missing(rules, goals), MissingFailed))
    }

    pub fn uninhabited(&self, sym: &SymRc) -> Result<bool, CoverageUninhabitedError> {
        use CoverageUninhabitedError::*;
        let typ = handle!(self.scope.normalize_typ(sym.typ()), NormalizeTypFailed);
        Ok(typ
            .head()
            .is_some_and(|head| self.scope.is_registered(head) && self.scope.constructors_of(head).is_empty()))
    }
}

#[derive(Error, Debug)]
pub enum CoverageCheckError {
    #[error("failed to instantiate function parameters")]
    ParametersFailed { source: ScopeParametersError },
    #[error("failed to expand function rules")]
    ExpandRuleFailed { source: ErrVec<CoverageExpandRuleError> },
    #[error("failed to check coverage")]
    MissingFailed { source: CoverageMissingError },
    #[error("rules for '{name}' leave uncovered arguments: {arguments}", name = sym.nym().identifier(), arguments = PlainRenderer::default().render_arguments(arguments))]
    CasesMissingInvalid { sym: SymRc, arguments: Vec<Exp> },
}

#[derive(Error, Debug)]
pub enum CoverageExpandRuleError {
    #[error("failed to expand argument patterns")]
    ExpandPatternFailed { source: ErrVec<ScopeExpandPatternError> },
}

#[derive(Error, Debug)]
pub enum CoverageMissingError {
    #[error("failed to compare rule patterns with the coverage case")]
    CompatibleFailed { source: ErrVec<CoverageCompatibleError> },
    #[error("failed to determine whether an argument type is empty")]
    UninhabitedFailed { source: ErrVec<CoverageUninhabitedError> },
    #[error("failed to normalize the split argument's type")]
    NormalizeTypFailed { source: ScopeNormalizeTypError },
    #[error("failed to check a constructor case")]
    BranchFailed { source: Box<CoverageBranchError> },
}

#[derive(Error, Debug)]
pub enum CoverageCompatibleError {
    #[error("failed to unify the rule with the coverage case")]
    ExpFailed { source: UnifierExpError },
}

#[derive(Error, Debug)]
pub enum CoverageBranchError {
    #[error("failed to instantiate the constructor")]
    SaturateFailed { source: ScopeSaturateError },
    #[error("failed to refine constructor indices")]
    TypFailed { source: UnifierTypError },
    #[error("failed to specialize the coverage case")]
    ExpFailed { source: UnifierExpError },
    #[error("failed to check the specialized coverage case")]
    MissingFailed { source: Box<CoverageMissingError> },
}

#[derive(Error, Debug)]
pub enum CoverageUninhabitedError {
    #[error("failed to normalize the argument type")]
    NormalizeTypFailed { source: ScopeNormalizeTypError },
}
