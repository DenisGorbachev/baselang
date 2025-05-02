use crate::{Of, TestPrelude};

#[test]
fn must_print_app() {
    let prelude = TestPrelude::new();
    let cons = prelude.list.cons;
    let nat = prelude.nat.nat;
    let cons_nat = cons.of(nat).unwrap();
    assert_eq!(cons_nat.print(true), "(cons nat) : (a : nat) -> list nat")
}

// #[test]
// fn must_print_app_in_complex_type() {
//     var!(t: Top);
//     var!(_test_var: typ!(t => Top));
// }
