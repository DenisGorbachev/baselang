use crate::{Bools, Of, PlainRenderer, Render, TestPrelude, assert_impl_of};
use std::sync::LazyLock;

pub static RENDERER: LazyLock<PlainRenderer> = LazyLock::new(PlainRenderer::default);

#[test]
fn must_render_app() {
    let prelude = TestPrelude::new();
    let cons = prelude.list.cons;
    let nat = prelude.nat.nat;
    let cons_nat = cons.of(nat).unwrap();
    let rendering = RENDERER.render_exp(&cons_nat).unwrap();
    assert_eq!(rendering, "(cons nat) : (a : nat) -> (o : list nat)")
}

#[test]
fn must_return_err_for_incorrect_application() {
    let (bool, yes, no) = Bools::into();
    assert!(bool.of(yes).is_err());
    assert!(bool.of(no).is_err());
}

#[test]
#[ignore]
fn must_return_ok_for_smart_application() {
    let prelude = TestPrelude::new();
    let cons = prelude.list.cons;
    let zero = prelude.nat.zero;
    let cons_bool_yes = cons
        .of_smart(zero)
        .expect("should find the argument position automatically");
    let rendering = RENDERER.render_exp(&cons_bool_yes).unwrap();
    assert_eq!(rendering, "(cons nat zero) : list nat")
}

#[test]
#[ignore]
fn must_assert_impl_of() {
    let prelude = TestPrelude::new();
    let cons = prelude.list.cons;
    let nat = prelude.nat.nat;
    assert_impl_of(&cons, nat);
}

// #[test]
// fn must_print_app_in_complex_type() {
//     var!(t: Top);
//     var!(_test_var: typ!(t => Top));
// }
mod macros;

pub use macros::*;
