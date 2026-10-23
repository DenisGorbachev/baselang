# Baselang concepts

## Cargo.toml

- All dependency versions and sources must be declared in `workspace.dependencies`.
- Package dependency declarations must use `workspace = true`.
- Package-specific features may be selected in package dependency declarations.
- `workspace.dependencies` must contain:
  - `derive_getters`

## Compiler type requirements

- Applies to `App`, `Sym`, `Def`, `Exp`, and `Typ`.
- Every struct field must have a distinct generic type.
- Recursive structs may use concrete field types when generic defaults would create recursive type aliases.
- Structs must derive `Getters`.
- Must not expose mutable access to fields that determine their type.
- Must keep cached type information immutable after construction.
- Must provide a `<Type>Rc` alias for `Rc<Type>`.
- Generic parameters must be preserved by the `<Type>Rc` alias.
- Shared references must clone the `Rc` instead of the underlying value.

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

- Must have fields:
  - `nym: Nym`
  - `typ: Typ`
  - `constructors: Constructors`
- Must pass [CTR](#compiler-type-requirements).
- Must compare symbol occurrences by identity using `Rc::ptr_eq`, with bound symbols compared through alpha-equivalence.
- Symbols with the same name and type may still have different identities.
- Must represent constructors such as `Nat`, `Zero`, and `Succ`.
- Must represent function parameters that may be bound and substituted.
- The `sym!` macro must construct symbols.

## struct Def

- Must have generics:
  - `N`, defaulting to `Nym`
  - `E`, defaulting to `Exp`
- Must have fields:
  - `nym: N`
  - `exp: E`
- Must pass [CTR](#compiler-type-requirements).
- `Def<N, Exp>` must have `pub fn typ(&self) -> &Typ` that returns `self.exp.typ()`.
- `Def<N, Exp>` must derive its type from the expression instead of storing a separate type.
- `Def<N, Exp>` must convert into `Exp` by returning its defining expression.
- Converting `&Def<N, Exp>` into `Exp` must clone the expression while preserving the identities of its symbols.
- `Def<N, Exp>` must convert into `Typ` as `Typ::One` of its defining expression.
- Application of `Def<N, Exp>` must use the defining expression as the function or argument.
- Definitions must not introduce function parameters or independent constructors.
- Must represent aliases such as `One = Succ Zero`.
- The `def!` macro must construct `Def<Nym, Exp>`.
- Renderers must distinguish symbol declarations (`Zero : Nat`) from definitions (`One = Succ Zero`).

## type Constructors

- Must be an alias for `Option<Vec<SymRc>>`.
- Constructor references must preserve the identities of the symbols used in expressions.
- `None` means that the symbol does not need constructors.
- `Some(vec![])` means that the symbol has no constructors.

## enum Typ

- Must have exactly the following variants:
  - `Top`,
  - `One(Exp)`
  - `Fun(DuoVec<SymRc>)`
- Must have methods:
  - `pub fn last(&self) -> &Self`

## enum Exp

- Must have exactly the following variants:
  - `Sol(SymRc)`
  - `App(ExpBox, ExpBox, TypBox)`
- Must have methods:
  - `pub fn app(fun: impl Into<Exp>, arg: impl Into<Exp>) -> Result<Self, InvalidApplicationError>`

## struct DuoVec

- Must wrap a `Vec<T>` containing at least two elements.
- Must have methods:
  - `pub fn first(&self) -> &T`
  - `pub fn last(&self) -> &T`

## .mise/tasks/spec/fix.sh

- Must run `cargo build --release --manifest-path spec/Cargo.toml` to build the `spec` package
- Must run `cargo fix` to check the primary package using the rustc wrapper from the `spec` package
  - Must set env vars:
    - `RUSTC_WORKSPACE_WRAPPER` (set it to the path to `spec` package main binary, which should be already built)

## Constructor of type X

- Must be a [sym](#struct-sym).
- Must satisfy `sym.typ().last() == X`.

Examples:

- `Zero : Nat` is a constructor of type `Nat`
- `Succ : Nat -> Nat` is a constructor of type `Nat`
- `Nil : (T : ^_^) -> List T` a constructor of type `List T`

## Constructor of sym S

- Must be a [sym](#struct-sym).
- Must satisfy `sym.typ().last().head() == S`.

Examples:

- `Zero : Nat` is a constructor of sym `Nat`
- `Succ : Nat -> Nat` is a constructor of sym `Nat`
- `Nil : (T : ^_^) -> List T` a constructor of sym `List` (because `List` is the head sym of `List T`) (note that `List T` is a type, not a sym)

## Smart application

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
