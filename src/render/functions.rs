use crate::Var;

pub fn print_vars<'a>(vars: impl IntoIterator<Item = &'a Var>) -> Vec<String> {
    vars.into_iter().map(Var::print).collect()
}

mod render_vars;
pub use render_vars::*;

mod dedup_renders;
pub use dedup_renders::*;

mod parse_prints;
pub use parse_prints::*;

mod render_module;
pub use render_module::*;

mod renderer_factory;
pub use renderer_factory::*;
