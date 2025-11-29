* Implement `Var::is_total` in @src/types/var.rs
  * Examples
    * Nat
    * Next
    * Sum
  * Counterexamples
    * Void
    * Weird (n : Nat) (_: Eq n Zero) (_ : Eq n (Next Zero))
    * _(a b c : Nat) (_ : Sum a b c) (_: Even a) (_ : Even b) (_ : Odd c)
