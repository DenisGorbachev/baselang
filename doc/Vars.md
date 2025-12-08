# Variables

* Not every definition can be made independent by moving the argument from ambient context into the definition
  * This is because in `PlusIndie/Zero` it's impossible to distinguish between `Zero` the constructor and `c` the variable
  * Example
  ```baselang
  # Define the type of natural numbers
  Nat : Type
  Zero : Nat
  Next : Nat -> Nat
  
  # Define addition with Nat from ambient context
  Plus : Nat -> Nat -> Nat -> Type
  Plus/Zero : (c : Nat) -> Plus Zero c c
  Plus/Next : (a b c : Nat) -> Plus a b c -> Plus (Next a) b (Next c)
  
  # Define addition without Nat from ambient context
  PlusIndie : (Nat : Type) -> T -> T -> T -> Type
  PlusIndie/Zero : (Nat : Type) -> (zero : Nat) -> (c : Nat) -> PlusIndie zero c c
  # No, this definitely doesn't work because `PlusIndie/Zero Nat (Next Zero) Zero : Plus (Next Zero) Zero Zero`, which doesn't capture the original idea of PlusIndie
  ```
* But it's possible to reuse the definitions that depend on vars from ambient context by replacing those specific vars
