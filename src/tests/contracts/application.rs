use crate::{AlphaEq, App, Exp, Scope, Sym, SymRc, Typ};
use crate::{ProbeApply, ProbeApplyApplyError};
use aist_core::{CallProbe, ContractReport, observe};
use std::ptr::{self, NonNull};
use std::rc::Rc;

pub fn application(report: &mut ContractReport) {
    let calls = Rc::new(CallProbe::default());
    let arg = Rc::new(());
    let typ = Rc::new(());
    let fun = ProbeApply {
        calls: calls.clone(),
        typ: Some(typ.clone()),
    };
    let app = observe!(report, App::new(fun, arg.clone()));
    report.require("App::new calls supplied fun with supplied arg exactly once", calls.once() && calls.last.get() == Some(NonNull::from(arg.as_ref())));
    report.require("App stores supplied fun, supplied arg and returned typ", Rc::ptr_eq(&app.fun().calls, &calls) && Rc::ptr_eq(app.arg(), &arg) && Rc::ptr_eq(app.typ(), &typ));
    let calls = Rc::new(CallProbe::default());
    let failed = App::new(
        ProbeApply {
            calls: calls.clone(),
            typ: None,
        },
        arg.clone(),
    );
    report.require("App forwards apply error", matches!(failed, Err(ProbeApplyApplyError::RejectedInvalid { arg: rejected }) if Rc::ptr_eq(&arg, &rejected)) && calls.once());
    let mut scope = Scope::default();
    let nat = observe!(report, scope.new_sym("Nat", Typ::Top));
    let zero = observe!(report, scope.new_sym("Zero", &nat));
    let param: SymRc = Sym::new_top_rc("T");
    let identity = observe!(report, scope.new_sym("F", Typ::fun(&param, &param)));
    let applied = observe!(report, scope.apply(&identity, &nat));
    report.require("dependent result substitutes the argument", applied.typ().alpha_eq(&Typ::one(&nat)));
    report.require("ill-typed argument is rejected", scope.apply(&identity, &zero).is_err());
    let raw = observe!(report, Exp::app(&identity, &nat));
    report.require("raw application substitutes result type", raw.typ().alpha_eq(applied.typ()));
    report.require("raw application rejects ill-typed argument", Exp::app(&identity, &zero).is_err());
    let alias = observe!(report, scope.define("Alias", &nat));
    let value = observe!(report, scope.new_sym("Value", &alias));
    let x: SymRc = Sym::new_rc("x", &nat);
    let successor = observe!(report, scope.new_sym("Succ", Typ::fun(&x, &nat)));
    report.require("application compares normalized argument types", scope.apply(&successor, &value).is_ok());
    let nested = Typ::fun(&param, Typ::fun(&x, &nat));
    report.require("last returns final codomain", nested.last().alpha_eq(&Typ::one(&nat)));
    [Typ::Top, Typ::one(&nat)]
        .iter()
        .for_each(|typ| report.require("last preserves non-function identity", ptr::eq(typ, typ.last())));
}
