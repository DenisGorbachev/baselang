use crate::{AlphaEq, Exp, Nym, SymRc, Typ, exp, sym, top, typ};
use aist_core::{ContractReport, EvaluationProbe, panic_message};
use std::panic::catch_unwind;
use std::rc::Rc;

pub fn macros(report: &mut ContractReport) {
    sym!(t);
    let _: &SymRc = &t;
    report.equal("sym introduces a named top-level symbol", t.nym().identifier(), "t");
    report.require("sym default type is Top", matches!(t.typ(), Typ::Top));
    let typ_probe = EvaluationProbe::default();
    sym!(x: typ_probe.evaluate(|| typ!(&t)));
    let _: &SymRc = &x;
    report.require("sym typed operand evaluates once", typ_probe.once() && x.typ().alpha_eq(&Typ::one(&t)));
    let typ_probe = EvaluationProbe::default();
    let nym_probe = EvaluationProbe::default();
    sym!(y: typ_probe.evaluate(|| &t); nym_probe.evaluate(|| Nym::from("custom")));
    let _: &SymRc = &y;
    report.require("sym accepts Into types and names exactly once", typ_probe.once() && nym_probe.once() && y.nym().identifier() == "custom");
    let top: Typ = top!();
    let typ: Typ = typ!();
    report.require("top and empty typ produce Top", matches!((top, typ), (Typ::Top, Typ::Top)));
    let operand = EvaluationProbe::default();
    let one: Typ = typ!(operand.evaluate(|| &t));
    report.require("typ expression evaluates once", operand.once() && one.alpha_eq(&Typ::one(&t)));
    let operand = EvaluationProbe::default();
    let fun: Typ = typ!(x => operand.evaluate(|| &t));
    report.require("typ function evaluates codomain once", operand.once() && fun.alpha_eq(&Typ::fun(&x, &t)));
    let operand = EvaluationProbe::default();
    let single: Exp = exp!(operand.evaluate(|| &x));
    report.require("single exp clones operand exactly once", operand.once() && Rc::ptr_eq(single.head(), &x));
    sym!(f: typ!(x => typ!(y => &t)));
    let first = EvaluationProbe::default();
    let second = EvaluationProbe::default();
    let third = EvaluationProbe::default();
    let application: Exp = exp!(first.evaluate(|| &f), second.evaluate(|| &x), third.evaluate(|| &y),);
    report.require("variadic exp evaluates each operand once and accepts trailing comma", first.once() && second.once() && third.once());
    report.require("exp associates to the left", matches!(application, Exp::App(ref fun, ref arg, _) if matches!(fun.as_ref(), Exp::App(..)) && Rc::ptr_eq(arg.head(), &y)));
    macro_failure(report);
}

pub fn macro_failure(report: &mut ContractReport) {
    let first = EvaluationProbe::default();
    let second = EvaluationProbe::default();
    let third = EvaluationProbe::default();
    let result = catch_unwind(|| {
        sym!(t);
        sym!(x: &t);
        exp!(first.evaluate(|| &x), second.evaluate(|| &t), third.evaluate(|| &x))
    });
    match result {
        Ok(_) => report.require("ill-typed exp must panic", false),
        Err(payload) => {
            let message = panic_message(payload.as_ref());
            report.require("exp panic includes explanation and Debug error", message.contains("should pass the type check") && message.contains("InvalidApplicationError"));
        }
    }
    report.require("earlier failed application prevents later operand evaluation", first.once() && second.once() && third.untouched());
}
