# Baselang concepts

## `Var`

A struct that represents a variable.

- Must have at least the following fields:
  - `nym: Nym`
  - `value: Value`
- Must have at least the following functions:
  - `pub fn typ(&self) -> &Typ`
    - Must match on `self.value`

Notes:

- Vars with a known value don't participate in application (because they already have a value)

## `Value`

An enum that represents a value of the variable.

- Must have exactly the following variants:
  - `Known(Exp)`
  - `Unknown(Typ)`

## `Typ`

An enum that represents a type of the variable.

- Must have exactly the following variants:
  - `Top`,
  - `One(Exp)`
  - `Fun(DuoVec<VarRc>)`
- Must have at least the following methods:
  - `pub fn last(&self) -> &Self`

## `Exp`

An enum that represents an expression.

- Must have exactly the following variants:
  - `Sol(VarRc)`
  - `App(ExpBox, ExpBox, TypBox)`
- Must have at least the following methods:
  - `pub fn app(fun: impl Into<Exp>, arg: impl Into<Exp>) -> Result<Self, InvalidApplicationError>`

## `DuoVec`

A newtype for `Vec` that has at least two elements.

- Must have at least the following methods:
  - `pub fn first(&self) -> &T`
  - `pub fn last(&self) -> &T`

## `.mise/tasks/spec/fix.sh`

- Must run `cargo build --release --manifest-path spec/Cargo.toml` to build the `spec` package
- Must run `cargo fix` to check the primary package using the rustc wrapper from the `spec` package
  - Must set env vars:
    - `RUSTC_WORKSPACE_WRAPPER` (set it to the path to `spec` package main binary, which should be already built)

## Constructor of type X

A [var](#var) whose `var.typ().last()` is `X`.

Examples:

- `Zero : Nat` is a constructor of type `Nat`
- `Succ : Nat -> Nat` is a constructor of type `Nat`
- `Nil : (T : ^_^) -> List T` a constructor of type `List T`

## Constructor of var V

A [var](#var) whose `var.typ().last().head()` is `V`.

Examples:

- `Zero : Nat` is a constructor of var `Nat`
- `Succ : Nat -> Nat` is a constructor of var `Nat`
- `Nil : (T : ^_^) -> List T` a constructor of var `List` (because `List` is the head var of `List T`) (note that `List T` is a type, not a var)

## Reducer of var V

A [var](#var) that satisfies the following requirements:

- `var.typ().prelast().head() == V`
- `var.typ().prelast() == var.typ().last()`
- The output must be strictly smaller than input (the recursion must be well-founded)
  - TODO: Specify this requirement more precisely (the current phrase conveys the intent, but is not technically correct)

Examples:

- `Add.Zero : (b : Nat) -> Add Zero b -> b` is a reducer of var `Add`
- `Add.Succ : (a : Nat) -> (b : Nat) -> Add (Succ a) b -> Succ (Add a b)` is a reducer of var `Add`

## Smart application

An algorithm for applying a function to an argument that fills some parameters automatically.

Algorithm:

- Find the first parameter that unifies with the argument
- Fill that parameter
- Fill preceding parameters recursively, working backwards, based on the unifications that were already performed
  - Fill parameters of type `^_^` from unifications
  - Fill parameters of other types either from unifications or from a map of canonical values of these types (similar to typeclasses)

Examples:

```baselang
F : (T : ^_^) -> (x : T)

// It is always valid to specify all arguments:
A1 : F Bool True

// It is valid to specify `True` because it is matched with x = True (which implies T = Bool):
A2 : F True 
```
