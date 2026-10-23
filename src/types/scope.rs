use crate::{AlphaEq, Constructors, Exp, Normalizer, NormalizerExpError, NormalizerTypError, Nym, Rewrite, Rewrites, Substitution, Sym, SymRc, Typ, Validator, ValidatorRuleError, is_identifier};
use derive_getters::Getters;
use errgonomic::{ErrVec, handle, handle_bool, handle_iter};
use std::collections::HashMap;
use std::ptr::NonNull;
use std::rc::Rc;
use thiserror::Error;

#[derive(Default, Getters, Clone, Debug)]
pub struct Scope<B = HashMap<String, SymRc>, R = Rewrites, C = Constructors> {
    bindings: B,
    rewrites: R,
    constructors: C,
}

pub type ScopeRc<B = HashMap<String, SymRc>, R = Rewrites, C = Constructors> = Rc<Scope<B, R, C>>;

impl Scope {
    pub fn apply_many(&self, fun: impl Into<Exp>, args: impl IntoIterator<Item = impl Into<Exp>>) -> Result<Exp, ScopeApplyManyError> {
        use ScopeApplyManyError::*;
        let fun = fun.into();
        let args = args.into_iter().map(Into::into).collect::<Vec<Exp>>();
        let result = args
            .iter()
            .try_fold(fun.clone(), |fun, arg| self.apply(fun, arg));
        Ok(handle!(result, ApplyFailed, fun, args))
    }

    pub fn parameters(&self, typ: &Typ) -> Result<Vec<SymRc>, ScopeParametersError> {
        use ScopeParametersError::*;
        let mut typ = handle!(self.normalize_typ(typ), NormalizeTypFailed);
        let mut parameters = Vec::new();
        while let Typ::Fun(param, body) = typ {
            parameters.push(param);
            typ = *body;
        }
        Ok(parameters)
    }

    pub fn saturate(&self, sym: &SymRc) -> Result<(Exp, Vec<SymRc>), ScopeSaturateError> {
        use ScopeSaturateError::*;
        let params = handle!(self.parameters(sym.typ()), ParametersFailed);
        let exp = handle!(
            params
                .iter()
                .try_fold(Exp::sol(sym), |fun, arg| self.apply(fun, arg)),
            ApplyFailed
        );
        Ok((exp, params))
    }

    pub fn new_sym(&mut self, nym: impl Into<Nym>, typ: impl Into<Typ>) -> Result<SymRc, ScopeNewSymError> {
        use ScopeNewSymError::*;
        let sym: SymRc = Sym::new_rc(nym, typ);
        Ok(handle!(self.insert_sym(sym), InsertSymFailed))
    }

    pub fn insert_sym(&mut self, sym: SymRc) -> Result<SymRc, ScopeInsertSymError> {
        use ScopeInsertSymError::*;
        handle_bool!(!is_identifier(sym.nym().identifier()), NymInvalid, sym);
        handle_bool!(self.bindings.contains_key(sym.nym().identifier()), NymOccupiedInvalid, sym);
        self.bindings
            .insert(sym.nym().identifier().to_owned(), sym.clone());
        let result = self.build_constructors();
        if result.is_err() {
            self.bindings.remove(sym.nym().identifier());
        }
        self.constructors = handle!(result, BuildConstructorsFailed, sym);
        Ok(sym)
    }

    pub fn define(&mut self, nym: impl Into<Nym>, body: impl Into<Exp>) -> Result<SymRc, ScopeDefineError> {
        use ScopeDefineError::*;
        let nym = nym.into();
        let body = body.into();
        handle_bool!(!is_identifier(nym.identifier()), NymInvalid, nym, body);
        handle_bool!(self.bindings.contains_key(nym.identifier()), NymOccupiedInvalid, nym, body);
        let typ = handle!(self.infer(&body), InferFailed, nym, body);
        let sym: SymRc = Sym::new_rc(nym, typ);
        self.bindings
            .insert(sym.nym().identifier().to_owned(), sym.clone());
        let result = self.insert_rewrite(sym.clone(), Rewrite::new(Vec::new(), body));
        if result.is_err() {
            self.bindings.remove(sym.nym().identifier());
        }
        handle!(result, InsertRewriteFailed);
        Ok(sym)
    }

    pub fn insert_rewrite(&mut self, sym: SymRc, rewrite: Rewrite) -> Result<(), ScopeInsertRewriteError> {
        use ScopeInsertRewriteError::*;
        handle_bool!(!self.is_registered(&sym), SymbolNotFound, sym, rewrite);
        handle_bool!(
            self.rewrites_of(&sym)
                .first()
                .is_some_and(|rule| rule.patterns.len() != rewrite.patterns.len()),
            ArityInvalid,
            sym,
            rewrite
        );
        let key = NonNull::from(sym.as_ref());
        self.rewrites.entry(key).or_default().push(rewrite.clone());
        let result = Validator::new(self).rule(&sym, &rewrite);
        let constructors = match &result {
            Ok(_) => Some(self.build_constructors()),
            Err(_) => None,
        };
        if (result.is_err() || constructors.as_ref().is_some_and(Result::is_err))
            && let Some(rules) = self.rewrites.get_mut(&key)
        {
            rules.pop();
            if rules.is_empty() {
                self.rewrites.remove(&key);
            }
        }
        handle!(result, RuleFailed, sym, rewrite);
        if let Some(constructors) = constructors {
            self.constructors = handle!(constructors, BuildConstructorsFailed, sym, rewrite);
        }
        Ok(())
    }

    pub fn sym(&self, name: &str) -> Option<&SymRc> {
        self.bindings.get(name)
    }

    pub fn is_registered(&self, sym: &SymRc) -> bool {
        self.bindings
            .get(sym.nym().identifier())
            .is_some_and(|registered| Rc::ptr_eq(registered, sym))
    }

    pub fn rewrites_of(&self, sym: &SymRc) -> &[Rewrite] {
        self.rewrites
            .get(&NonNull::from(sym.as_ref()))
            .map_or(&[], Vec::as_slice)
    }

    pub fn constructors_of(&self, sym: &SymRc) -> &[SymRc] {
        self.constructors
            .get(&NonNull::from(sym.as_ref()))
            .map_or(&[], Vec::as_slice)
    }

    pub fn normalize(&self, exp: &Exp) -> Result<Exp, ScopeNormalizeError> {
        use ScopeNormalizeError::*;
        Ok(handle!(Normalizer::new(self, false).exp(exp), ExpFailed))
    }

    pub fn normalize_typ(&self, typ: &Typ) -> Result<Typ, ScopeNormalizeTypError> {
        use ScopeNormalizeTypError::*;
        Ok(handle!(Normalizer::new(self, false).typ(typ), TypFailed))
    }

    pub fn expand_pattern(&self, exp: &Exp) -> Result<Exp, ScopeExpandPatternError> {
        use ScopeExpandPatternError::*;
        Ok(handle!(Normalizer::new(self, true).exp(exp), ExpFailed))
    }

    pub fn convertible(&self, left: &Typ, right: &Typ) -> Result<bool, ScopeConvertibleError> {
        use ScopeConvertibleError::*;
        let left = handle!(self.normalize_typ(left), NormalizeTypLeftFailed);
        let right = handle!(self.normalize_typ(right), NormalizeTypRightFailed);
        Ok(left.alpha_eq(&right))
    }

    pub fn apply(&self, fun: impl Into<Exp>, arg: impl Into<Exp>) -> Result<Exp, ScopeApplyError> {
        use ScopeApplyError::*;
        let fun = fun.into();
        let arg = arg.into();
        let fun_typ = handle!(self.normalize_typ(fun.typ()), NormalizeTypFailed, fun, arg);
        let Typ::Fun(param, result) = fun_typ else {
            return Err(FunctionInvalid {
                fun: Box::new(fun),
                arg: Box::new(arg),
            });
        };
        let valid = handle!(self.convertible(param.typ(), arg.typ()), ConvertibleFailed, fun, arg);
        let expected = param.typ().clone();
        handle_bool!(!valid, ArgumentInvalid, fun, arg, expected);
        let substitution = Substitution::from([(NonNull::from(param.as_ref()), arg.clone())]);
        let typ = result.instantiate(&substitution);
        Ok(Exp::App(Box::new(fun), Box::new(arg), Box::new(typ)))
    }

    pub fn infer(&self, exp: &Exp) -> Result<Typ, ScopeInferError> {
        use ScopeInferError::*;
        match exp {
            Exp::Sol(sym) => Ok(sym.typ().clone()),
            Exp::App(fun, arg, _) => {
                let fun_typ = handle!(self.infer(fun), InferFunFailed);
                let arg_typ = handle!(self.infer(arg), InferArgFailed);
                let fun_typ = handle!(self.normalize_typ(&fun_typ), NormalizeTypFailed);
                let Typ::Fun(param, body) = fun_typ else {
                    return Err(FunctionInvalid {
                        exp: Box::new(exp.clone()),
                    });
                };
                let valid = handle!(self.convertible(param.typ(), &arg_typ), ConvertibleFailed);
                let exp_owned = exp.clone();
                handle_bool!(!valid, ArgumentInvalid, exp: exp_owned);
                let substitution = Substitution::from([(NonNull::from(param.as_ref()), arg.as_ref().clone())]);
                Ok(body.instantiate(&substitution))
            }
        }
    }

    pub fn check_rewrite(&self, sym: &SymRc, rewrite: &Rewrite) -> Result<(), ScopeCheckRewriteError> {
        use ScopeCheckRewriteError::*;
        let lhs = handle!(
            rewrite
                .patterns
                .iter()
                .try_fold(Exp::sol(sym), |fun, arg| self.apply(fun, arg)),
            ApplyFailed
        );
        let body_typ = handle!(self.infer(&rewrite.body), InferFailed);
        let valid = handle!(self.convertible(lhs.typ(), &body_typ), ConvertibleFailed);
        let expected = lhs.typ().clone();
        handle_bool!(!valid, BodyTypeInvalid, expected, actual: body_typ);
        Ok(())
    }

    pub fn build_constructors(&self) -> Result<Constructors, ScopeBuildConstructorsError> {
        use ScopeBuildConstructorsError::*;
        let entries = self
            .bindings
            .values()
            .filter(|sym| self.rewrites_of(sym).is_empty())
            .map(|sym| {
                use ScopeConstructorHeadError::*;
                let sym = sym.clone();
                let typ = handle!(self.normalize_typ(sym.typ().last()), NormalizeTypFailed, sym);
                Ok((typ.head().cloned(), sym))
            });
        let entries = handle_iter!(entries, ConstructorHeadFailed);
        Ok(entries
            .into_iter()
            .fold(Constructors::new(), |mut constructors, (head, sym)| {
                if let Some(head) = head {
                    constructors
                        .entry(NonNull::from(head.as_ref()))
                        .or_default()
                        .push(sym);
                }
                constructors
            }))
    }
}

#[derive(Error, Debug)]
pub enum ScopeApplyManyError {
    #[error("failed to apply successive arguments")]
    ApplyFailed { source: ScopeApplyError, fun: Box<Exp>, args: Vec<Exp> },
}

#[derive(Error, Debug)]
pub enum ScopeParametersError {
    #[error("failed to normalize the parameter telescope")]
    NormalizeTypFailed { source: ScopeNormalizeTypError },
}

#[derive(Error, Debug)]
pub enum ScopeSaturateError {
    #[error("failed to instantiate the constructor parameters")]
    ParametersFailed { source: ScopeParametersError },
    #[error("failed to apply the constructor to its fresh parameters")]
    ApplyFailed { source: ScopeApplyError },
}

#[derive(Error, Debug)]
pub enum ScopeNewSymError {
    #[error("failed to register the new symbol")]
    InsertSymFailed { source: ScopeInsertSymError },
}

#[derive(Error, Debug)]
pub enum ScopeInsertSymError {
    #[error("'{name}' is not a valid identifier", name = sym.nym().identifier())]
    NymInvalid { sym: SymRc },
    #[error("name '{name}' is already bound", name = sym.nym().identifier())]
    NymOccupiedInvalid { sym: SymRc },
    #[error("failed to rebuild constructor indexes")]
    BuildConstructorsFailed { source: ScopeBuildConstructorsError, sym: SymRc },
}

#[derive(Error, Debug)]
pub enum ScopeDefineError {
    #[error("'{name}' is not a valid identifier", name = nym.identifier())]
    NymInvalid { nym: Box<Nym>, body: Box<Exp> },
    #[error("name '{name}' is already bound", name = nym.identifier())]
    NymOccupiedInvalid { nym: Box<Nym>, body: Box<Exp> },
    #[error("failed to infer the definition's type")]
    InferFailed { source: ScopeInferError, nym: Box<Nym>, body: Box<Exp> },
    #[error("failed to register the definition's rule")]
    InsertRewriteFailed { source: ScopeInsertRewriteError },
}

#[derive(Error, Debug)]
pub enum ScopeInsertRewriteError {
    #[error("symbol '{name}' is not registered", name = sym.nym().identifier())]
    SymbolNotFound { sym: SymRc, rewrite: Rewrite },
    #[error("inconsistent rule arity for '{name}'", name = sym.nym().identifier())]
    ArityInvalid { sym: SymRc, rewrite: Rewrite },
    #[error("invalid rule for '{name}'", name = sym.nym().identifier())]
    RuleFailed { source: ValidatorRuleError, sym: SymRc, rewrite: Rewrite },
    #[error("failed to rebuild constructor indexes")]
    BuildConstructorsFailed { source: ScopeBuildConstructorsError, sym: SymRc, rewrite: Rewrite },
}

#[derive(Error, Debug)]
pub enum ScopeNormalizeError {
    #[error("failed to normalize the expression")]
    ExpFailed { source: NormalizerExpError },
}

#[derive(Error, Debug)]
pub enum ScopeNormalizeTypError {
    #[error("failed to normalize the type")]
    TypFailed { source: NormalizerTypError },
}

#[derive(Error, Debug)]
pub enum ScopeExpandPatternError {
    #[error("failed to unfold zero-argument definitions in the pattern")]
    ExpFailed { source: NormalizerExpError },
}

#[derive(Error, Debug)]
pub enum ScopeConvertibleError {
    #[error("failed to normalize the expected type")]
    NormalizeTypLeftFailed { source: ScopeNormalizeTypError },
    #[error("failed to normalize the actual type")]
    NormalizeTypRightFailed { source: ScopeNormalizeTypError },
}

#[derive(Error, Debug)]
pub enum ScopeApplyError {
    #[error("failed to normalize the function type")]
    NormalizeTypFailed { source: ScopeNormalizeTypError, fun: Box<Exp>, arg: Box<Exp> },
    #[error("failed to compare argument types")]
    ConvertibleFailed { source: ScopeConvertibleError, fun: Box<Exp>, arg: Box<Exp> },
    #[error("the applied expression does not have a function type")]
    FunctionInvalid { fun: Box<Exp>, arg: Box<Exp> },
    #[error("argument type differs from the parameter type")]
    ArgumentInvalid { fun: Box<Exp>, arg: Box<Exp>, expected: Box<Typ> },
}

#[derive(Error, Debug)]
pub enum ScopeInferError {
    #[error("failed to infer the function type")]
    InferFunFailed { source: Box<Self> },
    #[error("failed to infer the argument type")]
    InferArgFailed { source: Box<Self> },
    #[error("failed to normalize the function type")]
    NormalizeTypFailed { source: ScopeNormalizeTypError },
    #[error("failed to compare argument types")]
    ConvertibleFailed { source: ScopeConvertibleError },
    #[error("the applied expression does not have a function type")]
    FunctionInvalid { exp: Box<Exp> },
    #[error("argument type differs from the parameter type")]
    ArgumentInvalid { exp: Box<Exp> },
}

#[derive(Error, Debug)]
pub enum ScopeCheckRewriteError {
    #[error("failed to apply the rule head to its patterns")]
    ApplyFailed { source: ScopeApplyError },
    #[error("failed to infer the rule body's type")]
    InferFailed { source: ScopeInferError },
    #[error("failed to compare the rule's types")]
    ConvertibleFailed { source: ScopeConvertibleError },
    #[error("rule body has a different type from its left-hand side")]
    BodyTypeInvalid { expected: Box<Typ>, actual: Box<Typ> },
}

#[derive(Error, Debug)]
pub enum ScopeBuildConstructorsError {
    #[error("failed to determine constructor heads")]
    ConstructorHeadFailed { source: ErrVec<ScopeConstructorHeadError> },
}

#[derive(Error, Debug)]
pub enum ScopeConstructorHeadError {
    #[error("failed to normalize the result type of '{name}'", name = sym.nym().identifier())]
    NormalizeTypFailed { source: ScopeNormalizeTypError, sym: SymRc },
}
