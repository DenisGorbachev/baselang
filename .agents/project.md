# Baselang concepts

## Cargo.toml

- All dependency versions and sources must be declared in `workspace.dependencies`.
- Package dependency declarations must use `workspace = true`.
- Package-specific features may be selected in package dependency declarations.
- `workspace.dependencies` must contain:
  - `derive_getters`

## Compiler type requirements

- Every field must have a distinct generic type.
- Must have a `Getters` derive

Alias: CTR.

## trait Apply

- Must have associated types:
  - `Arg`
  - `Typ`
  - `Err: std::error::Error`
- Must have `fn apply(&self, arg: &Self::Arg) -> Result<Self::Typ, Self::Err>`.
- `apply` must validate the argument and return the application's resulting type.

## struct App

- Must have generics:
  - `F`
  - `A`
  - `T`
- Must have fields:
  - `fun: F`
  - `arg: A`
  - `typ: T`
- Must pass [CTR](#compiler-type-requirements).

### impl<F, A, T> App<F, A, T>

#### pub fn new

- Must have args:
  - `fun: F`
  - `arg: A`
- Must return `Result<Self, F::Err>`.
- Must require `F: Apply<Arg = A, Typ = T>`.
- Must compute `typ` by calling `fun.apply(&arg)`.

## struct Sym

A struct that introduces an identity with a type and no defining expression.

- Must have fields:
  - `nym: Nym`
  - `typ: Typ`
  - `constructors: Constructors`
- Must derive `Getters`.
- Must keep its type immutable after construction.
- Must be shared through `SymRc`, an alias for `Rc<Sym>`.
- Must compare symbol occurrences by identity using `Rc::ptr_eq`, with bound symbols compared through alpha-equivalence.
- Symbols with the same name and type may still have different identities.
- Includes constructors such as `Nat`, `Zero`, and `Succ`.
- Includes function parameters that may be bound and substituted.
- The `sym!` macro must construct symbols.

## struct Def

A struct that names an expression without introducing a new symbol identity.

- Must have fields:
  - `nym: Nym`
  - `exp: Exp`
- Must derive `Getters`.
- Must have `pub fn typ(&self) -> &Typ` that returns `self.exp.typ()`.
- Must derive its type from the expression instead of storing a separate type.
- Must convert into `Exp` by returning its defining expression.
- Converting `&Def` into `Exp` must clone the expression while preserving the identities of its symbols.
- Must convert into `Typ` as `Typ::One` of its defining expression.
- Application must use the defining expression as the function or argument.
- Definitions must not introduce function parameters or independent constructors.
- Includes aliases such as `One = Succ Zero`.
- The `def!` macro must construct definitions.
- Renderers must distinguish symbol declarations (`Zero : Nat`) from definitions (`One = Succ Zero`).

## type Constructors

An alias for `Option<Vec<Sym>>`.

- `None` means that the symbol does not need constructors.
- `Some(vec![])` means that the symbol has no constructors.

## enum Typ

An enum that represents a type of a symbol or expression.

- Must have exactly the following variants:
  - `Top`,
  - `One(Exp)`
  - `Fun(DuoVec<SymRc>)`
- Must have methods:
  - `pub fn last(&self) -> &Self`

## enum Exp

An enum that represents an expression.

- Must have exactly the following variants:
  - `Sol(SymRc)`
  - `App(ExpBox, ExpBox, TypBox)`
- Must have methods:
  - `pub fn app(fun: impl Into<Exp>, arg: impl Into<Exp>) -> Result<Self, InvalidApplicationError>`

## struct DuoVec

A newtype for `Vec` that has at least two elements.

- Must have methods:
  - `pub fn first(&self) -> &T`
  - `pub fn last(&self) -> &T`

## .mise/tasks/spec/fix.sh

- Must run `cargo build --release --manifest-path spec/Cargo.toml` to build the `spec` package
- Must run `cargo fix` to check the primary package using the rustc wrapper from the `spec` package
  - Must set env vars:
    - `RUSTC_WORKSPACE_WRAPPER` (set it to the path to `spec` package main binary, which should be already built)

## Constructor of type X

A [sym](#struct-sym) whose `sym.typ().last()` is `X`.

Examples:

- `Zero : Nat` is a constructor of type `Nat`
- `Succ : Nat -> Nat` is a constructor of type `Nat`
- `Nil : (T : ^_^) -> List T` a constructor of type `List T`

## Constructor of sym V

A [sym](#struct-sym) whose `sym.typ().last().head()` is `V`.

Examples:

- `Zero : Nat` is a constructor of sym `Nat`
- `Succ : Nat -> Nat` is a constructor of sym `Nat`
- `Nil : (T : ^_^) -> List T` a constructor of sym `List` (because `List` is the head sym of `List T`) (note that `List T` is a type, not a sym)

## Reducer of sym V

A [sym](#struct-sym) that satisfies the following requirements:

- `sym.typ().prelast().head() == V`
- `sym.typ().prelast() == sym.typ().last()`
- The output must be strictly smaller than input (the recursion must be well-founded)
  - TODO: Specify this requirement more precisely (the current phrase conveys the intent, but is not technically correct)

Examples:

- `Add.Zero : (b : Nat) -> Add Zero b -> b` is a reducer of sym `Add`
- `Add.Succ : (a : Nat) -> (b : Nat) -> Add (Succ a) b -> Succ (Add a b)` is a reducer of sym `Add`

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
