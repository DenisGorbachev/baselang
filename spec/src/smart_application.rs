use crate::{BOOLS, same_exp};
use aist_core::{ContractReport, observe};
use baselang::{Of, PlainParser};

pub fn smart_application(report: &mut ContractReport) {
    [
        ("type inference", "F : (T : Type) -> (x : T) -> Type", "True", "F Bool True"),
        ("first matching parameter", "F : Bool -> Bool -> Bool", "True", "F True"),
        ("canonical omitted value", "Unit : Type\nOnly : Unit\nF : Unit -> Bool -> Type", "True", "F Only True"),
        ("dependent omitted value", "P : Bool -> Type\nEvidence : P True\nF : (x : Bool) -> P x -> Type", "Evidence", "F True Evidence"),
    ]
    .into_iter()
    .for_each(|(name, declarations, arg, expected)| {
        report.run(name, |report| smart_application_case(report, declarations, arg, expected));
    });
}

pub fn smart_application_case(report: &mut ContractReport, declarations: &str, arg: &str, expected: &str) {
    let parser = PlainParser::default();
    let source = format!("{BOOLS}\n{declarations}");
    let scope = observe!(report, parser.parse_scope(&source));
    let fun = observe!(report, parser.expression("F", &scope));
    let arg = observe!(report, parser.expression(arg, &scope));
    let expected = observe!(report, parser.expression(expected, &scope));
    let actual = fun.of_smart(arg);
    report.require("elaboration equals explicit application", actual.is_some_and(|actual| same_exp(&actual, &expected)));
}
