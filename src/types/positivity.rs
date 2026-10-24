use crate::{ConstructorDeclaration, Exp, PositivityEdge, Scope, ScopeNormalizeTypError, Sym, SymRc, Typ};
use errgonomic::{ErrVec, handle, handle_iter};
use std::collections::HashMap;
use std::ptr::NonNull;
use std::rc::Rc;
use thiserror::Error;

pub struct Positivity<'a> {
    scope: &'a Scope,
    constructors: Vec<ConstructorDeclaration>,
    parameters: HashMap<NonNull<Sym>, Vec<bool>>,
}

impl<'a> Positivity<'a> {
    pub fn new(scope: &'a Scope) -> Result<Self, PositivityNewError> {
        use PositivityNewError::*;
        let declarations = handle_iter!(
            scope.bindings().values().map(|sym| {
                use PositivityNormalizeDeclarationError::*;
                let sym = sym.clone();
                let typ = handle!(scope.normalize_typ(sym.typ()), NormalizeTypFailed, sym);
                Ok((sym, typ))
            }),
            NormalizeDeclarationFailed
        );
        let parameters = declarations
            .iter()
            .map(|(sym, typ)| {
                let strict = scope.rewrites_of(sym).is_empty();
                (NonNull::from(sym.as_ref()), typ.parameters().map(|_| strict).collect())
            })
            .collect();
        let constructors = declarations
            .into_iter()
            .filter_map(|(sym, typ)| {
                let family = typ
                    .last()
                    .head()
                    .filter(|family| scope.rewrites_of(&sym).is_empty() && scope.is_registered(family))
                    .cloned();
                family.map(|family| ConstructorDeclaration::new(family, sym, typ))
            })
            .collect();
        Ok(Self {
            scope,
            constructors,
            parameters,
        })
    }

    pub fn check(&mut self) -> Result<(), PositivityCheckError> {
        use PositivityCheckError::*;
        while self.refine_parameters() {}
        let edges = self.edges();
        let mut outgoing = HashMap::<NonNull<Sym>, Vec<&PositivityEdge>>::new();
        edges.iter().for_each(|edge| {
            outgoing
                .entry(NonNull::from(edge.from.as_ref()))
                .or_default()
                .push(edge);
        });
        let failures = edges.iter().filter(|edge| !edge.strict).map(|edge| {
            use PositivityCycleError::*;
            match edge.cycle(&outgoing) {
                Some(cycle) => Err(CycleInvalid {
                    edge: edge.clone(),
                    cycle,
                }),
                None => Ok(()),
            }
        });
        handle_iter!(failures, CycleFailed);
        Ok(())
    }

    pub fn refine_parameters(&mut self) -> bool {
        let checker = &*self;
        let rejected = checker
            .parameters
            .iter()
            .flat_map(|(family, parameters)| {
                parameters
                    .iter()
                    .enumerate()
                    .filter_map(move |(index, strict)| (*strict && !checker.parameter_is_strict(*family, index)).then_some((*family, index)))
            })
            .collect::<Vec<_>>();
        rejected.iter().for_each(|(family, index)| {
            if let Some(strict) = self
                .parameters
                .get_mut(family)
                .and_then(|parameters| parameters.get_mut(*index))
            {
                *strict = false;
            }
        });
        !rejected.is_empty()
    }

    pub fn parameter_is_strict(&self, family: NonNull<Sym>, index: usize) -> bool {
        self.constructors
            .iter()
            .filter(|declaration| NonNull::from(declaration.family.as_ref()) == family)
            .all(|declaration| {
                let Typ::One(result) = declaration.typ.last() else { return false };
                let (_, args) = result.spine();
                let Some(Exp::Sol(parameter)) = args.get(index).copied() else { return false };
                if !declaration
                    .typ
                    .parameters()
                    .any(|bound| Rc::ptr_eq(bound, parameter))
                {
                    return false;
                }
                if args
                    .iter()
                    .enumerate()
                    .any(|(other, arg)| other != index && arg.contains_sym(parameter))
                {
                    return false;
                }
                let mut strict = true;
                declaration.typ.parameters().for_each(|field| {
                    self.visit_typ(field.typ(), true, &mut |sym, occurrence_strict| {
                        if Rc::ptr_eq(sym, parameter) && !occurrence_strict {
                            strict = false;
                        }
                    })
                });
                strict
            })
    }

    pub fn visit_typ(&self, typ: &Typ, strict: bool, visit: &mut impl FnMut(&SymRc, bool)) {
        match typ {
            Typ::Top => (),
            Typ::One(exp) => self.visit_exp(exp, strict, visit),
            Typ::Fun(param, body) => {
                self.visit_typ(param.typ(), false, visit);
                self.visit_typ(body, strict, visit);
            }
        }
    }

    pub fn visit_exp(&self, exp: &Exp, strict: bool, visit: &mut impl FnMut(&SymRc, bool)) {
        let (head, args) = exp.spine();
        visit(head, strict);
        let parameters = self.parameters.get(&NonNull::from(head.as_ref()));
        args.into_iter().enumerate().for_each(|(index, arg)| {
            let argument_strict = parameters
                .and_then(|parameters| parameters.get(index))
                .copied()
                .unwrap_or(false);
            self.visit_exp(arg, strict && argument_strict, visit);
        });
    }

    pub fn edges(&self) -> Vec<PositivityEdge> {
        let mut edges = Vec::new();
        self.constructors.iter().for_each(|declaration| {
            declaration.typ.parameters().for_each(|field| {
                self.visit_typ(field.typ(), true, &mut |sym, strict| {
                    if self.scope.is_registered(sym) {
                        edges.push(PositivityEdge::new(declaration.family.clone(), sym.clone(), declaration.constructor.clone(), strict));
                    }
                })
            });
        });
        edges
    }
}

#[derive(Error, Debug)]
pub enum PositivityNewError {
    #[error("failed to normalize declarations for positivity checking")]
    NormalizeDeclarationFailed { source: ErrVec<PositivityNormalizeDeclarationError> },
}

#[derive(Error, Debug)]
pub enum PositivityNormalizeDeclarationError {
    #[error("failed to normalize the type of '{name}'", name = sym.nym().identifier())]
    NormalizeTypFailed { source: ScopeNormalizeTypError, sym: SymRc },
}

#[derive(Error, Debug)]
pub enum PositivityCheckError {
    #[error("constructor dependencies are not strictly positive")]
    CycleFailed { source: ErrVec<PositivityCycleError> },
}

#[derive(Error, Debug)]
pub enum PositivityCycleError {
    #[error("constructor '{constructor}' of '{family}' uses '{target}' in a non-strictly-positive recursive dependency", constructor = edge.constructor.nym().identifier(), family = edge.from.nym().identifier(), target = edge.to.nym().identifier())]
    CycleInvalid { edge: PositivityEdge, cycle: Vec<PositivityEdge> },
}
