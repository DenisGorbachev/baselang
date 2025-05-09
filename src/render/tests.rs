use crate::{CueRenderer, Module, Nat, Of, PlainRenderer, Render, TestPrelude, render_module};

#[test]
fn must_render_app() {
    let prelude = TestPrelude::new();
    let cons = prelude.list.cons;
    let nat = prelude.nat.nat;
    let cons_nat = cons.of(nat).unwrap();
    assert_eq!(cons_nat.print(true), "(cons nat) : (a : nat) -> list nat")
}

#[test]
#[ignore]
fn debug_cue_renderer() {
    let module = Nat::new();
    let renderer = CueRenderer::new();
    debug_renderer_output(&module, &renderer);

    let plain_renderer = PlainRenderer::new();
    debug_renderer_output(&module, &plain_renderer);
}

// #[test]
// fn must_print_app_in_complex_type() {
//     var!(t: Top);
//     var!(_test_var: typ!(t => Top));
// }
mod debug_renderer;
mod macros;

pub use debug_renderer::*;
pub use macros::*;
