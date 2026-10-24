use aist_core::ContractReport;

pub fn check() -> ContractReport {
    let mut report = ContractReport::default();
    report.run("structure", declarations);
    report.run("application", application);
    report.run("registration", registration);
    report.run("rewrites", rewrites);
    report.run("rollback", rollback);
    report.run("normalization", normalization);
    report.run("constructors", constructors);
    report.run("theory", theory);
    report.run("parser", parser);
    report.run("macros", macros);
    report.run("rendering", rendering);
    report.run("smart application", smart_application);
    report.unverified("CTR/universal preservation", "for every constructor, conversion, deserializer and public mutation: invariant(pre) implies invariant(post), on both Ok and Err; finite executions do not prove this quantification");
    report.unverified("Apply/universal validity", "for every implementation F and every fun and arg: apply(fun, arg) = Ok(typ) iff some typ satisfies V_F(fun, arg, typ), and every returned typ satisfies V_F");
    report.unverified("DuoVec/invariant preservation", "self.0.len() >= 2; first() and last() reference the corresponding endpoints; every constructor and mutation must preserve this predicate");
    report
}

mod declarations;
pub use declarations::*;
mod application;
pub use application::*;
mod registration;
pub use registration::*;
mod rewrites;
pub use rewrites::*;
mod normalization;
pub use normalization::*;
mod theory;
pub use theory::*;
mod parser;
pub use parser::*;
mod macros;
pub use macros::*;
mod rendering;
pub use rendering::*;
mod smart_application;
pub use smart_application::*;
mod scope_invariants;
pub use scope_invariants::*;
mod constants;
pub use constants::*;
mod probe_apply;
pub use probe_apply::*;
mod constructors;
pub use constructors::*;
mod rollback;
pub use rollback::*;
mod same_scope;
pub use same_scope::*;
mod same_exp;
pub use same_exp::*;
