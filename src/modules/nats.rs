use crate::{Exp, Module, Nym, NymEn, NymLang, NymRu, NymRuCases, RefsTuple4, Rewrite, Scope, ScopeApplyManyError, ScopeInsertRewriteError, ScopeInsertSymError, SymRc, Validator, ValidatorValidateError, impl_syms_vec, sym, typ};
use derive_more::Into;
use errgonomic::{ErrVec, handle, handle_iter};
use thiserror::Error;

#[derive(Into, Clone, Debug)]
pub struct Nats {
    pub nat: SymRc,
    pub zero: SymRc,
    pub next: SymRc,
    pub add: SymRc,
    pub scope: Scope,
}

pub type NatsTuple = (SymRc, SymRc, SymRc, SymRc, Scope);

impl Nats {
    pub fn new() -> Result<Self, NatsNewError> {
        use NatsNewError::*;
        sym!(nat: typ!(); Self::nat_nym());
        sym!(zero: typ!(&nat));
        sym!(n: typ!(&nat));
        sym!(next: typ!(n => &nat));
        sym!(a: typ!(&nat));
        sym!(b: typ!(&nat));
        sym!(add: typ!(a => typ!(b => &nat)));
        let mut scope = Scope::default();
        let registrations = [&nat, &zero, &next, &add]
            .into_iter()
            .map(|sym| scope.insert_sym(sym.clone()));
        handle_iter!(registrations, InsertSymFailed, scope);
        let zero_rule = Rewrite::new(vec![Exp::sol(&zero), Exp::sol(&b)], Exp::sol(&b));
        let rule_head = add.clone();
        handle!(scope.insert_rewrite(rule_head, zero_rule), InsertRewriteZeroFailed, scope);
        let next_a = handle!(scope.apply_many(&next, [&a]), ApplyManyNextFailed, scope);
        let add_a_b = handle!(scope.apply_many(&add, [&a, &b]), ApplyManyAddFailed, scope);
        let body = handle!(scope.apply_many(&next, [add_a_b]), ApplyManyBodyFailed, scope);
        let next_rule = Rewrite::new(vec![next_a, Exp::sol(&b)], body);
        let rule_head = add.clone();
        handle!(scope.insert_rewrite(rule_head, next_rule), InsertRewriteNextFailed, scope);
        handle!(Validator::new(&scope).validate(), ValidateFailed, scope);
        Ok(Self {
            nat,
            zero,
            next,
            add,
            scope,
        })
    }

    pub fn nat_nym() -> Nym {
        Nym {
            short: NymLang {
                en: NymEn::from("nat"),
                ru: NymRu {
                    singular: NymRuCases::new("нат", "ната", "нату", "нат", "натом", "нате"),
                    plural: NymRuCases::new("наты", "наты", "натам", "натов", "натами", "натах"),
                }
                .into(),
            },
            long: NymLang {
                en: NymEn::from("natural number"),
                ru: NymRu {
                    singular: NymRuCases::new("натуральное число", "натурального числа", "натуральному числу", "натуральное число", "натуральным числом", "натуральном числе"),
                    plural: NymRuCases::new("натуральные числа", "натуральных чисел", "натуральным числам", "натуральные числа", "натуральными числами", "натуральных числах"),
                }
                .into(),
            }
            .into(),
        }
    }
}

impl_syms_vec!(Nats, nat, zero, next, add; scope);

impl Module for Nats {
    type RefsTuple<'a> = RefsTuple4<'a>;

    fn refs_tuple(&self) -> Self::RefsTuple<'_> {
        (&self.nat, &self.zero, &self.next, &self.add)
    }
}

#[derive(Error, Debug)]
pub enum NatsNewError {
    #[error("failed to register natural-number symbols")]
    InsertSymFailed { source: ErrVec<ScopeInsertSymError>, scope: Box<Scope> },
    #[error("failed to insert the zero case of addition")]
    InsertRewriteZeroFailed { source: ScopeInsertRewriteError, scope: Box<Scope> },
    #[error("failed to construct the successor pattern")]
    ApplyManyNextFailed { source: ScopeApplyManyError, scope: Box<Scope> },
    #[error("failed to construct recursive addition")]
    ApplyManyAddFailed { source: ScopeApplyManyError, scope: Box<Scope> },
    #[error("failed to construct the addition result")]
    ApplyManyBodyFailed { source: ScopeApplyManyError, scope: Box<Scope> },
    #[error("failed to insert the successor case of addition")]
    InsertRewriteNextFailed { source: ScopeInsertRewriteError, scope: Box<Scope> },
    #[error("invalid natural-number module")]
    ValidateFailed { source: ValidatorValidateError, scope: Box<Scope> },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PlainRenderer, assert_eq_prints};

    #[test]
    fn must_print() -> Result<(), MustPrintError> {
        use MustPrintError::*;
        let module = handle!(Nats::new(), NewFailed);
        assert_eq_prints(&module, &PlainRenderer::default(), include_str!("nats/prints/plain.base"));
        Ok(())
    }
    #[derive(Error, Debug)]
    pub enum MustPrintError {
        #[error("failed to construct natural numbers")]
        NewFailed { source: NatsNewError },
    }
}
