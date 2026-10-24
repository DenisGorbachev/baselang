use crate::{BOOLS, same_exp, same_scope};
use crate::{Parse, PlainParser, PlainParserParseScopeError, PlainParserParseScopeLineError};
use aist_core::{ContractReport, observe};

pub fn parser(report: &mut ContractReport) {
    let mut parser = PlainParser::default();
    let source = format!("{BOOLS}\nNot : Bool -> Bool\nNot True => False\nNot False => True");
    let provisional = observe!(report, parser.parse_scope(&source));
    let theory = observe!(report, parser.parse(&source));
    report.equal("parse retains all declarations", theory.scope().bindings().len(), provisional.bindings().len());
    report.equal(
        "parse retains all rewrite rules",
        theory
            .scope()
            .rewrites()
            .values()
            .map(Vec::len)
            .sum::<usize>(),
        2,
    );
    let actual = observe!(report, parser.expression("Not True", theory.scope()));
    let normalized = observe!(report, theory.scope().normalize(&actual));
    let expected = observe!(report, parser.expression("False", theory.scope()));
    report.require("expression resolves supplied scope", same_exp(&normalized, &expected));
    report.require("expression checks applications in supplied scope", parser.expression("Not Bool", theory.scope()).is_err());
    let incomplete = format!("{BOOLS}\nNot : Bool -> Bool\nNot True => False");
    report.require("parse_scope permits incomplete coverage", parser.parse_scope(&incomplete).is_ok());
    report.require("parse validates before success", parser.parse(&incomplete).is_err());
    [
        "Bool : Type\nTrue : Missing".to_owned(),
        format!("{BOOLS}\nNot : Bool -> Bool\nNot True => Missing"),
    ]
    .into_iter()
    .for_each(|source| {
        let failed = parser.parse_scope(&source);
        let expected_line = source.lines().count();
        report.require("parse error retains source and failing line", matches!(failed, Err(PlainParserParseScopeError::ParseScopeLineFailed { input, source: error }) if input == source && matches!(*error, PlainParserParseScopeLineError::ParseLineFailed { line_number, .. } if line_number == expected_line)));
    });
    let before = theory.scope().clone();
    observe!(report, parser.expression("Not True", theory.scope()));
    report.require("expression lookup does not mutate scope", same_scope(theory.scope(), &before));
}
