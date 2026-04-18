# Baselang concepts

## `.mise/tasks/spec/fix.sh`

- Must run `cargo build --release --manifest-path spec/Cargo.toml` to build the `spec` package
- Must run `cargo fix` to check the primary package using the rustc wrapper from the `spec` package
  - Must set env vars:
    - `RUSTC_WORKSPACE_WRAPPER` (set it to the path to `spec` package main binary, which should be already built)

## `Var`

A struct that represents a variable.

- Must have at least the following fields:
  - `value: Value`
- Must have at least the following functions:
  - `pub fn typ(&self) -> &Typ`
    - Must match on `self.value`

Notes:

- Vars with a known value don't participate in application (because they already have a value)

## `Value`

An enum that represents a value of the variable.

- Must have exactly the following variants:
  - `Known { exp: Exp }`
  - `Unknown { typ: Typ }`

## `Typ`

An enum that represents a type of the variable.

- Must have exactly the following variants:
  - `Top`,
  - `One { exp: Exp }`
  - `Fun { vars: DuoVec<VarRc> }`

## `Exp`

An enum that represents an expression.

- Must have exactly the following variants:
  - `Sol { var: VarRc }`
  - `App { fun: ExpBox, arg: ExpBox, typ: TypBox }`
- Must have at least the following methods:
  - `pub fn app(fun: impl Into<Exp>, arg: impl Into<Exp>) -> Result<Self, InvalidApplicationError>`

## `DuoVec`

A newtype for `Vec` that has at least two elements.

- Must have at least the following methods:
  - `pub fn first(&self) -> &T`
  - `pub fn last(&self) -> &T`
