use crate::{AlphaEq, App, Exp, Scope, Sym, SymRc, Typ};
use crate::{InvalidApplicationError, ScopeApplyError, ScopeDefineError, ScopeNewSymError};
use crate::{ProbeApply, ProbeApplyApplyError};
use errgonomic::handle;
use std::cell::Cell;
use std::ptr::{self, NonNull};
use std::rc::Rc;
use thiserror::Error;

#[test]
fn must_preserve_application_contract() -> Result<(), MustPreserveApplicationContractError> {
    use MustPreserveApplicationContractError::*;
    let calls = Rc::new(Cell::new(None));
    let arg = Rc::new(());
    let typ = Rc::new(());
    let fun = ProbeApply {
        calls: calls.clone(),
        typ: Some(typ.clone()),
    };
    let result = App::new(fun, arg.clone());
    let app = handle!(result, NewFailed);
    assert!(calls.get() == Some(NonNull::from(arg.as_ref())), "App::new calls supplied fun with supplied arg exactly once");
    assert!(Rc::ptr_eq(&app.fun().calls, &calls) && Rc::ptr_eq(app.arg(), &arg) && Rc::ptr_eq(app.typ(), &typ), "App stores supplied fun, supplied arg and returned typ");
    let calls = Rc::new(Cell::new(None));
    let failed = App::new(
        ProbeApply {
            calls: calls.clone(),
            typ: None,
        },
        arg.clone(),
    );
    assert!(matches!(failed, Err(ProbeApplyApplyError::RejectedInvalid { arg: rejected }) if Rc::ptr_eq(&arg, &rejected)) && calls.get().is_some(), "App forwards apply error");
    let mut scope = Scope::default();
    let nat = handle!(scope.new_sym("Nat", Typ::Top), NewSymFailed);
    let zero = handle!(scope.new_sym("Zero", &nat), NewSymFailed);
    let param: SymRc = Sym::new_top_rc("T");
    let identity = handle!(scope.new_sym("F", Typ::fun(&param, &param)), NewSymFailed);
    let applied = handle!(scope.apply(&identity, &nat), ApplyFailed);
    assert!(applied.typ().alpha_eq(&Typ::one(&nat)), "dependent result substitutes the argument");
    assert!(scope.apply(&identity, &zero).is_err(), "ill-typed argument is rejected");
    let raw = handle!(Exp::app(&identity, &nat), AppFailed);
    assert!(raw.typ().alpha_eq(applied.typ()), "raw application substitutes result type");
    assert!(Exp::app(&identity, &zero).is_err(), "raw application rejects ill-typed argument");
    let alias = handle!(scope.define("Alias", &nat), DefineFailed);
    let value = handle!(scope.new_sym("Value", &alias), NewSymFailed);
    let x: SymRc = Sym::new_rc("x", &nat);
    let successor = handle!(scope.new_sym("Succ", Typ::fun(&x, &nat)), NewSymFailed);
    assert!(scope.apply(&successor, &value).is_ok(), "application compares normalized argument types");
    let nested = Typ::fun(&param, Typ::fun(&x, &nat));
    assert!(nested.last().alpha_eq(&Typ::one(&nat)), "last returns final codomain");
    [Typ::Top, Typ::one(&nat)]
        .iter()
        .for_each(|typ| assert!(ptr::eq(typ, typ.last()), "last preserves non-function identity"));
    Ok(())
}

#[derive(Error, Debug)]
pub enum MustPreserveApplicationContractError {
    #[error("failed to call new")]
    NewFailed { source: ProbeApplyApplyError },
    #[error("failed to call app")]
    AppFailed { source: InvalidApplicationError },
    #[error("failed to call new_sym")]
    NewSymFailed { source: ScopeNewSymError },
    #[error("failed to call define")]
    DefineFailed { source: ScopeDefineError },
    #[error("failed to call apply")]
    ApplyFailed { source: ScopeApplyError },
}
