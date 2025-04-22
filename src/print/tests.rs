use crate::{Of, TestPrelude};

#[test]
#[ignore]
fn must_print_app() {
    let prelude = TestPrelude::default();
    let cons = prelude.list.cons;
    let nat = prelude.nat.nat;
    let cons_nat = cons.of(nat).unwrap();
    assert_eq!(cons_nat.print(true), "(cons nat) : (a : nat) -> list nat")
}
