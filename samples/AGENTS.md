# Baselang

Baselang is a language for resolving disputes between people.

The disputes are resolved by formal logic. Every person expresses their beliefs in Baselang, then asks the compiler to check them. The compiler performs two checks:

- Syntactic (makes sure that definitions are well-formed)
- Semantic (makes sure that definitions don't contradict each other)

The semantic check tries to find a proof of `Bullshit` (a built-in type with no constructors) (aka `False`, `Void`, `Bottom`). If the compiler finds a proof of `Bullshit`, it presents the full proof to the person who wrote it.

Baselang implements a non-standard type theory:

- `^_^` is a special "unknown type" (like `None` in Rust, `Nothing` in Haskell, `null` in TypeScript)
  - Given `A : ^_^` and `B : ^_^`, it is not valid to conclude that `A` and `B` are constructors of the same type (because `^_^` is an "unknown type").
  - Given `A : T` and `B : T`, it is valid to conclude that `A` and `B` are constructors of the same type (because `T` is a known type).
- There is no universe hierarchy (the Girard paradox is avoided because a term is not allowed to be a type of itself, i.e. `T : T` is not allowed)
- Any term can be both a value or a type
  - `T : ^_^; A : T; B : A;` is valid code (notice that `A` is both a type and a value)
- Function types are standard (`A -> B` is a type of functions from `A` to `B`)
- Function values are non-standard:
  - Match arms are defined as function types, for example:
    ```baselang
    Add : (a : Nat) -> (b : Nat) -> (c : Nat)
    Add.Zero : (b : Nat) -> (Add Zero b -> b)
    Add.Succ : (a : Nat) -> (b : Nat) -> (Add (Succ a) b -> Succ (Add a b))
    ```
  - Totality and termination checks are deferred until call-time, so it's possible to add / remove match arms to existing functions (in general: it's possible to add / remove constructors for existing types)
