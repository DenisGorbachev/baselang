use crate::evaluate_once;
use crate::{AlphaEq, Exp, Nym, SymRc, Typ, exp, sym, top, typ};
use std::panic::catch_unwind;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use thiserror::Error;

#[test]
fn must_preserve_macros_contract() -> Result<(), MustPreserveMacrosContractError> {
    sym!(t);
    let _: &SymRc = &t;
    assert_eq!(t.nym().identifier(), "t", "sym introduces a named top-level symbol");
    assert!(matches!(t.typ(), Typ::Top), "sym default type is Top");
    let typ_probe = AtomicBool::default();
    sym!(x: evaluate_once(&typ_probe, || typ!(&t)));
    let _: &SymRc = &x;
    assert!(typ_probe.load(Relaxed) && x.typ().alpha_eq(&Typ::one(&t)), "sym typed operand evaluates once");
    let typ_probe = AtomicBool::default();
    let nym_probe = AtomicBool::default();
    sym!(y: evaluate_once(&typ_probe, || &t); evaluate_once(&nym_probe, || Nym::from("custom")));
    let _: &SymRc = &y;
    assert!(typ_probe.load(Relaxed) && nym_probe.load(Relaxed) && y.nym().identifier() == "custom", "sym accepts Into types and names exactly once");
    let top: Typ = top!();
    let typ: Typ = typ!();
    assert!(matches!((top, typ), (Typ::Top, Typ::Top)), "top and empty typ produce Top");
    let operand = AtomicBool::default();
    let one: Typ = typ!(evaluate_once(&operand, || &t));
    assert!(operand.load(Relaxed) && one.alpha_eq(&Typ::one(&t)), "typ expression evaluates once");
    let operand = AtomicBool::default();
    let fun: Typ = typ!(x => evaluate_once(&operand, || &t));
    assert!(operand.load(Relaxed) && fun.alpha_eq(&Typ::fun(&x, &t)), "typ function evaluates codomain once");
    let operand = AtomicBool::default();
    let single: Exp = exp!(evaluate_once(&operand, || &x));
    assert!(operand.load(Relaxed) && Rc::ptr_eq(single.head(), &x), "single exp clones operand exactly once");
    sym!(f: typ!(x => typ!(y => &t)));
    let first = AtomicBool::default();
    let second = AtomicBool::default();
    let third = AtomicBool::default();
    let application: Exp = exp!(evaluate_once(&first, || &f), evaluate_once(&second, || &x), evaluate_once(&third, || &y),);
    assert!(first.load(Relaxed) && second.load(Relaxed) && third.load(Relaxed), "variadic exp evaluates each operand once and accepts trailing comma");
    assert!(matches!(application, Exp::App(ref fun, ref arg, _) if matches!(fun.as_ref(), Exp::App(..)) && Rc::ptr_eq(arg.head(), &y)), "exp associates to the left");
    assert_exp_macro_failure();
    Ok(())
}

pub fn assert_exp_macro_failure() {
    let first = AtomicBool::default();
    let second = AtomicBool::default();
    let third = AtomicBool::default();
    let result = catch_unwind(|| {
        sym!(t);
        sym!(x: &t);
        exp!(evaluate_once(&first, || &x), evaluate_once(&second, || &t), evaluate_once(&third, || &x))
    });
    assert!(
        result.is_err_and(|payload| {
            let message = payload
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| payload.downcast_ref::<&str>().copied());
            message.is_some_and(|message| message.contains("should pass the type check") && message.contains("InvalidApplicationError"))
        }),
        "ill-typed exp must panic with an explanation and Debug error"
    );
    assert!(first.load(Relaxed) && second.load(Relaxed) && !third.load(Relaxed), "earlier failed application prevents later operand evaluation");
}

#[derive(Error, Copy, Clone, Debug)]
pub enum MustPreserveMacrosContractError {}
