use crate::{Exp, Module, Nats, RefsTuple4, Rewrite, Scope, ScopeApplyManyError, ScopeInsertRewriteError, ScopeInsertSymError, SymRc, Typ, Validator, ValidatorValidateError, impl_syms_vec, sym, typ};
use derive_more::Into;
use errgonomic::{ErrVec, handle, handle_iter};
use thiserror::Error;

#[derive(Into, Clone, Debug)]
pub struct Vectors {
    pub vector: SymRc,
    pub nil: SymRc,
    pub cons: SymRc,
    pub append: SymRc,
    pub scope: Scope,
}

pub type VectorsTuple = (SymRc, SymRc, SymRc, SymRc, Scope);

impl Vectors {
    pub fn new(nats: &Nats) -> Result<Self, VectorsNewError> {
        use VectorsNewError::*;
        let Nats {
            nat,
            zero,
            next,
            add,
            ..
        } = nats;
        let mut scope = nats.scope.clone();
        sym!(t: Typ::Top);
        sym!(len: typ!(nat));
        sym!(vector: typ!(t => typ!(len => Typ::Top)));
        let vector_t_zero = handle!(scope.apply_many(&vector, [&t, zero]), ApplyVectorZeroFailed, scope);
        sym!(nil: typ!(t => vector_t_zero));
        sym!(head: typ!(&t));
        let vector_t_len = handle!(scope.apply_many(&vector, [&t, &len]), ApplyVectorLengthFailed, scope);
        sym!(tail: typ!(vector_t_len));
        let next_len = handle!(scope.apply_many(next, [&len]), ApplyNextLengthFailed, scope);
        let vector_t_next = handle!(scope.apply_many(&vector, [Exp::sol(&t), next_len]), ApplyVectorNextFailed, scope);
        sym!(cons: typ!(t => typ!(len => typ!(head => typ!(tail => vector_t_next)))));
        sym!(len_a: typ!(nat));
        sym!(len_b: typ!(nat));
        let vector_t_a = handle!(scope.apply_many(&vector, [&t, &len_a]), ApplyVectorLeftFailed, scope);
        let vector_t_b = handle!(scope.apply_many(&vector, [&t, &len_b]), ApplyVectorRightFailed, scope);
        sym!(a: typ!(vector_t_a.clone()));
        sym!(b: typ!(vector_t_b));
        let add_lengths = handle!(scope.apply_many(add, [&len_a, &len_b]), ApplyAddLengthsFailed, scope);
        let result_arguments = [Exp::sol(&t), add_lengths.clone()];
        let result = handle!(scope.apply_many(&vector, result_arguments), ApplyResultTypeFailed, scope);
        sym!(append: typ!(t => typ!(len_a => typ!(len_b => typ!(a => typ!(b => result))))));
        let registrations = [&vector, &nil, &cons, &append]
            .into_iter()
            .map(|sym| scope.insert_sym(sym.clone()));
        handle_iter!(registrations, InsertSymFailed, scope);
        let nil_t = handle!(scope.apply_many(&nil, [&t]), ApplyNilFailed, scope);
        let rule = Rewrite::new(
            vec![
                Exp::sol(&t),
                Exp::sol(zero),
                Exp::sol(&len_b),
                nil_t,
                Exp::sol(&b),
            ],
            Exp::sol(&b),
        );
        let rule_head = append.clone();
        handle!(scope.insert_rewrite(rule_head, rule), InsertRewriteNilFailed, scope);
        sym!(head: typ!(&t));
        sym!(tail: typ!(vector_t_a));
        let next_len_a = handle!(scope.apply_many(next, [&len_a]), ApplyNextLeftFailed, scope);
        let cons_pattern = handle!(scope.apply_many(&cons, [&t, &len_a, &head, &tail]), ApplyConsPatternFailed, scope);
        let recursive = handle!(scope.apply_many(&append, [&t, &len_a, &len_b, &tail, &b]), ApplyRecursiveFailed, scope);
        let body = handle!(scope.apply_many(&cons, [Exp::sol(&t), add_lengths, Exp::sol(&head), recursive]), ApplyBodyFailed, scope);
        let rule = Rewrite::new(
            vec![
                Exp::sol(&t),
                next_len_a,
                Exp::sol(&len_b),
                cons_pattern,
                Exp::sol(&b),
            ],
            body,
        );
        let rule_head = append.clone();
        handle!(scope.insert_rewrite(rule_head, rule), InsertRewriteConsFailed, scope);
        handle!(Validator::new(&scope).validate(), ValidateFailed, scope);
        Ok(Self {
            vector,
            nil,
            cons,
            append,
            scope,
        })
    }
}

impl_syms_vec!(Vectors, vector, nil, cons, append; scope);

impl Module for Vectors {
    type RefsTuple<'a> = RefsTuple4<'a>;

    fn refs_tuple(&self) -> Self::RefsTuple<'_> {
        (&self.vector, &self.nil, &self.cons, &self.append)
    }
}

#[derive(Error, Debug)]
pub enum VectorsNewError {
    #[error("failed to construct the empty vector type")]
    ApplyVectorZeroFailed { source: ScopeApplyManyError, scope: Box<Scope> },
    #[error("failed to construct the vector element type")]
    ApplyVectorLengthFailed { source: ScopeApplyManyError, scope: Box<Scope> },
    #[error("failed to construct the successor length")]
    ApplyNextLengthFailed { source: ScopeApplyManyError, scope: Box<Scope> },
    #[error("failed to construct the successor vector type")]
    ApplyVectorNextFailed { source: ScopeApplyManyError, scope: Box<Scope> },
    #[error("failed to construct the left vector type")]
    ApplyVectorLeftFailed { source: ScopeApplyManyError, scope: Box<Scope> },
    #[error("failed to construct the right vector type")]
    ApplyVectorRightFailed { source: ScopeApplyManyError, scope: Box<Scope> },
    #[error("failed to construct the sum of vector lengths")]
    ApplyAddLengthsFailed { source: ScopeApplyManyError, scope: Box<Scope> },
    #[error("failed to construct append's result type")]
    ApplyResultTypeFailed { source: ScopeApplyManyError, scope: Box<Scope> },
    #[error("failed to register vector symbols")]
    InsertSymFailed { source: ErrVec<ScopeInsertSymError>, scope: Box<Scope> },
    #[error("failed to construct the empty vector pattern")]
    ApplyNilFailed { source: ScopeApplyManyError, scope: Box<Scope> },
    #[error("failed to insert append's empty-vector rule")]
    InsertRewriteNilFailed { source: ScopeInsertRewriteError, scope: Box<Scope> },
    #[error("failed to construct the successor length pattern")]
    ApplyNextLeftFailed { source: ScopeApplyManyError, scope: Box<Scope> },
    #[error("failed to construct the nonempty vector pattern")]
    ApplyConsPatternFailed { source: ScopeApplyManyError, scope: Box<Scope> },
    #[error("failed to construct recursive append")]
    ApplyRecursiveFailed { source: ScopeApplyManyError, scope: Box<Scope> },
    #[error("failed to construct append's result")]
    ApplyBodyFailed { source: ScopeApplyManyError, scope: Box<Scope> },
    #[error("failed to insert append's nonempty-vector rule")]
    InsertRewriteConsFailed { source: ScopeInsertRewriteError, scope: Box<Scope> },
    #[error("invalid vector module")]
    ValidateFailed { source: ValidatorValidateError, scope: Box<Scope> },
}
