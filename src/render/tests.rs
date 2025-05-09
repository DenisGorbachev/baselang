use crate::{Of, PlainRenderer, Render, TestPrelude};

#[test]
fn must_render_app() {
    let prelude = TestPrelude::new();
    let cons = prelude.list.cons;
    let nat = prelude.nat.nat;
    let cons_nat = cons.of(nat).unwrap();
    let renderer = PlainRenderer::default();
    assert_eq!(renderer.render_exp(&cons_nat), Some("(cons nat) : (a : nat) -> list nat".to_string()))
}

// #[test]
// fn must_print_app_in_complex_type() {
//     var!(t: Top);
//     var!(_test_var: typ!(t => Top));
// }
mod macros;

pub use macros::*;
