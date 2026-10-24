use crate::BOOLS;
use aist_core::{ContractReport, observe};
use baselang::{PlainParser, PlainRenderer, Render, Rewrite};

pub fn rendering(report: &mut ContractReport) {
    let parser = PlainParser::default();
    let scope = observe!(report, parser.parse_scope(BOOLS));
    let renderer = PlainRenderer::new("Type");
    scope.bindings().values().for_each(|sym| {
        let name = sym.nym().short.en.singular.to_string();
        let typ = renderer.render_typ_inner(sym.typ());
        report.equal("render_sym equation", renderer.render_sym(sym), Some(format!("{name} : {typ}")));
    });
    let truth = observe!(report, parser.expression("True", &scope));
    let rule = Rewrite::new(Vec::new(), truth.clone());
    report.equal("zero-argument rewrite equation", renderer.render_rewrite(truth.head(), &rule), Some("True => True".to_owned()));
}
