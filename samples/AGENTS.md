# Baselang

Baselang is a language for resolving disputes between people.

The disputes are resolved by formal logic. Every person expresses their beliefs in Baselang, then asks the compiler to check them. The compiler performs two checks:

- Syntactic (makes sure that definitions are well-formed)
- Semantic (makes sure that definitions don't contradict each other)

The semantic check tries to find a proof of `Bullshit` (a built-in type with no constructors) (aka `False`, `Void`, `Bottom`). If the compiler finds a proof of `Bullshit`, it presents the full proof to the person who wrote it.

## Baselang type theory

- Every term has a type.
- Some terms have a special type `^_^`, which means "unknown":
  - `^_^` itself doesn't have a type (`^_^` is not a term).
  - Given `A : ^_^` and `B : ^_^`, it is not valid to conclude that `A` and `B` are constructors of the same type (because `^_^` means "unknown").
  - Given `A : T` and `B : T`, it is valid to conclude that `A` and `B` are constructors of the same type (because `T` is known).
- Every term is both a value and a type:
  - `T : ^_^; A : T; B : A;` is valid code (notice that `A` is both a value and a type).
- No term can be mentioned in a type of itself or its ancestors in the type hierarchy:
  - `T : T` cycle is not allowed.
  - `A : B; B : A` cycle is not allowed.
  - `T : T -> ^_^` mention is not allowed.
  - `A : B; B : A -> ^_^` mention is not allowed.
- There is no universe hierarchy (it is replaced by a type hierarchy, because every term is both a value and a type)
- Some terms have function types (e.g. `A -> B` is a type of functions from `A` to `B`).
  - Function types don't have types (they are not terms).
- Some terms have rewrites (that's how functions are defined):
  - Match arms are represented as function types, for example:
    ```baselang
    Add : (a : Nat) -> (b : Nat) -> (c : Nat)
    Add.Zero : (b : Nat) -> (Add Zero b -> b)
    Add.Succ : (a : Nat) -> (b : Nat) -> (Add (Succ a) b -> Succ (Add a b))
    ```
- Totality, positivity, termination, productivity checks are performed only after the compiler finishes reading the terms
  - It's possible to add / remove constructors for existing types
  - It's possible to add / remove match arms to existing functions
- Evaluation of expressions doesn't change their type (so preservation holds).
- Data are values of type `^_^` (e.g. `Nat : ^_^`, `List : (T : ^_^) -> ^_^`)
- Propositions are values of type `^_^` (e.g. `Eq : (T : ^_^) -> (a : T) -> (b : T) -> ^_^`)
  - There is no distinction between `Type` and `Prop`.
