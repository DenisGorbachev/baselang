use crate::{AssertScopeInvariantsError, ScopeNewSymError};
use crate::{Nym, Scope, ScopeInsertSymError, Sym, SymRc, Typ};
use crate::{assert_scope_invariants, same_scope};
use errgonomic::{ErrVec, handle, handle_iter};
use std::rc::Rc;
use thiserror::Error;

#[test]
fn must_preserve_registration_contract() -> Result<(), MustPreserveRegistrationContractError> {
    use MustPreserveRegistrationContractError::*;
    let mut scope: Scope = Scope::default();
    assert!(scope.bindings().is_empty() && scope.rewrites().is_empty() && scope.constructors().is_empty(), "default scope is empty");
    let raw: SymRc = Sym::new_top_rc("Raw");
    assert!(scope.bindings().is_empty(), "raw symbol construction does not register");
    assert!(scope.rewrites_of(&raw).is_empty() && scope.constructors_of(&raw).is_empty(), "unregistered identities have no rules or constructors");
    let result = scope.insert_sym(raw.clone());
    let registered = handle!(result, InsertSymFailed);
    assert!(Rc::ptr_eq(&raw, &registered), "insert_sym returns the supplied allocation");
    let rejected: SymRc = Sym::new_top_rc("Raw");
    let before = scope.clone();
    assert!(matches!(scope.insert_sym(rejected.clone()), Err(ScopeInsertSymError::NymOccupiedInvalid { sym }) if Rc::ptr_eq(&sym, &rejected)), "occupied identifier returns the rejected symbol");
    assert!(same_scope(&scope, &before), "failed registration leaves scope unchanged");
    ["", ".A", "A.", "A..B", "1A", "A-B", "A.1B", " A", "A B"]
        .into_iter()
        .for_each(|name| {
            let before = scope.clone();
            assert!(scope.new_sym(name, Typ::Top).is_err(), "reject identifier {name:?}");
            assert!(same_scope(&scope, &before), "invalid identifier leaves scope unchanged");
        });
    let results = ["A", "_", "_A", "A0", "A.B", "Α.β", "名.字"]
        .into_iter()
        .map(|name| assert_registered_identifier(&mut scope, name));
    handle_iter!(results, AssertRegisteredIdentifierFailed);
    let parameter: SymRc = Sym::new_top_rc("parameter");
    handle!(scope.new_sym("Function", Typ::fun(&parameter, Typ::Top)), NewSymFailed);
    assert!(scope.sym("parameter").is_none(), "binding a function parameter does not register it");
    handle!(assert_scope_invariants(&scope), AssertScopeInvariantsFailed);
    Ok(())
}

pub fn assert_registered_identifier(scope: &mut Scope, name: &str) -> Result<(), AssertRegisteredIdentifierError> {
    use AssertRegisteredIdentifierError::*;
    let sym = handle!(scope.new_sym(name, Typ::Top), NewSymFailed);
    assert!(
        scope
            .sym(name)
            .is_some_and(|registered| Rc::ptr_eq(registered, &sym)),
        "new_sym returns a registered symbol"
    );
    let nym = Nym::from(name);
    assert_eq!(nym.identifier(), nym.short.en.singular.canonical.as_str(), "Nym::identifier selects canonical singular English");
    Ok(())
}

#[derive(Error, Debug)]
pub enum MustPreserveRegistrationContractError {
    #[error("failed to call new_sym")]
    NewSymFailed { source: ScopeNewSymError },
    #[error("failed to call insert_sym")]
    InsertSymFailed { source: ScopeInsertSymError },
    #[error("failed to call assert_scope_invariants")]
    AssertScopeInvariantsFailed { source: AssertScopeInvariantsError },
    #[error("failed to call assert_registered_identifier")]
    AssertRegisteredIdentifierFailed { source: ErrVec<AssertRegisteredIdentifierError> },
}

#[derive(Error, Debug)]
pub enum AssertRegisteredIdentifierError {
    #[error("failed to call new_sym")]
    NewSymFailed { source: ScopeNewSymError },
}
