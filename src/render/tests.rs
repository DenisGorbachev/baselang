use crate::{Bools, Of, PlainRenderer, Render, TestPrelude, TestPreludeNewError, assert_impl_of};
use errgonomic::handle;
use std::sync::LazyLock;
use thiserror::Error;

pub static RENDERER: LazyLock<PlainRenderer> = LazyLock::new(PlainRenderer::default);

#[test]
fn must_render_app() -> Result<(), MustRenderAppError> {
    use MustRenderAppError::*;
    let prelude = handle!(TestPrelude::new(), NewFailed);
    let cons = prelude.list.cons;
    let nat = prelude.nat.nat;
    let cons_nat = cons.of(nat).unwrap();
    let rendering = RENDERER.render_exp(&cons_nat).unwrap();
    assert_eq!(rendering, "(cons nat) : (a : nat) -> list nat");
    Ok(())
}

#[test]
fn must_return_err_for_incorrect_application() {
    let (bool, yes, no) = Bools::into();
    assert!(bool.of(yes).is_err());
    assert!(bool.of(no).is_err());
}

#[test]
#[ignore]
fn must_return_ok_for_smart_application() -> Result<(), MustReturnOkForSmartApplicationError> {
    use MustReturnOkForSmartApplicationError::*;
    let prelude = handle!(TestPrelude::new(), NewFailed);
    let cons = prelude.list.cons;
    let zero = prelude.nat.zero;
    let cons_bool_yes = cons
        .of_smart(zero)
        .expect("should find the argument position automatically");
    let rendering = RENDERER.render_exp(&cons_bool_yes).unwrap();
    assert_eq!(rendering, "(cons nat zero) : list nat");
    Ok(())
}

#[test]
#[ignore]
fn must_assert_impl_of() -> Result<(), MustAssertImplOfError> {
    use MustAssertImplOfError::*;
    let prelude = handle!(TestPrelude::new(), NewFailed);
    let cons = prelude.list.cons;
    let nat = prelude.nat.nat;
    assert_impl_of(&cons, nat);
    Ok(())
}

// #[test]
// fn must_print_app_in_complex_type() {
//     sym!(t: Top);
//     sym!(_test_sym: typ!(t => Top));
// }
#[derive(Error, Debug)]
pub enum MustRenderAppError {
    #[error("failed to construct the test prelude")]
    NewFailed { source: TestPreludeNewError },
}

#[derive(Error, Debug)]
pub enum MustReturnOkForSmartApplicationError {
    #[error("failed to construct the test prelude")]
    NewFailed { source: TestPreludeNewError },
}

#[derive(Error, Debug)]
pub enum MustAssertImplOfError {
    #[error("failed to construct the test prelude")]
    NewFailed { source: TestPreludeNewError },
}

mod macros;

pub use macros::*;
