# Baselang concepts

## Cargo.toml

- All dependency versions and sources must be declared in `workspace.dependencies`.
- Package dependency declarations must use `workspace = true`.
- Package-specific features may be selected in package dependency declarations.
- `workspace.dependencies` must contain:
  - `derive_getters`

## Compiler type requirements

- Must have an [Rc alias](#rc-alias-of-type-t).
- Must state each invariant as a predicate over its fields or the results of specified methods.
- Every constructor, conversion, and deserializer that returns a value of the type must establish all declared invariants before returning that value.
- Every public mutator must preserve all declared invariants when called on a value satisfying them.
- Preservation of invariants must hold on both successful and error returns from a mutator.
- A public field or an API exposing mutable access to a field is permitted only if every assignment or mutation available through that access preserves all declared invariants.

Alias: CTR.

## Rc alias of type T

- Must be a public type alias whose name is `T`'s name followed by `Rc`.
- Must declare the same generic parameters, in the same order and with the same defaults, as `T`.
- Must alias `Rc<T>` with each generic argument forwarded to the corresponding parameter of `T`.

## Compiler struct requirements

- Must be a struct.
- Must pass [CTR](#compiler-type-requirements).
- The declared type of each field must be a type parameter of the struct.
- Different fields must use different type parameters in their declarations.
- Must derive `Getters`.

Alias: CSR.

## Macro constructor requirements

- Must be declared with `macro_rules!`.
- Must have `#[macro_export]`.
- References to items from this crate in its expansion must use `$crate` paths.
- Must support every input form specified in the type's section.
- Each accepted input form must specify whether the expansion is an expression or a binding declaration.
- Each expression form must specify its result type.
- Each binding form must specify the identifier and type of the introduced binding.
- Every supplied expression operand must be evaluated at most once.
- If the expansion completes without unwinding, every supplied expression operand must have been evaluated exactly once.
- Every form that invokes a fallible function must specify its behavior for both `Ok` and `Err`.
- A form with a variadic operand list must accept a trailing comma.

## trait Apply

- Must have associated types:
  - `Arg`
  - `Typ`
  - `Err: std::error::Error`
- Must have `fn apply(&self, arg: &Self::Arg) -> Result<Self::Typ, Self::Err>`.
- Each implementation for a type `F` must specify a validity predicate `V_F(fun, arg, typ)`, where `fun: F`, `arg: F::Arg`, and `typ: F::Typ`.
- `apply` must return `Ok(typ)` only if `V_F(self, arg, typ)` holds.
- If no `typ` satisfies `V_F(self, arg, typ)`, `apply` must return `Err`.
- If at least one `typ` satisfies `V_F(self, arg, typ)`, `apply` must return one such `typ` in `Ok`.

## struct App

- Must have generics:
  - `F`
  - `A`
  - `T`
- Must have exactly these fields:
  - `fun: F`
  - `arg: A`
  - `typ: T`
- Must pass [CSR](#compiler-struct-requirements).
- Invariant: `V_F(fun, arg, typ)` must hold whenever `F: Apply<Arg = A, Typ = T>`.

### impl<F, A, T> App<F, A, T>

#### pub fn new

- Must have args:
  - `fun: F`
  - `arg: A`
- Must return `Result<Self, F::Err>`.
- Must require `F: Apply<Arg = A, Typ = T>`.
- Must call `fun.apply(&arg)` exactly once.
- If that call returns `Ok(typ)`, must return `Ok(Self { fun, arg, typ })` using the supplied `fun` and `arg` and the returned `typ`.
- If that call returns `Err(error)`, must return `Err(error)`.

## struct Sym

- Must have generics:
  - `N`, defaulting to `Nym`
  - `T`, defaulting to `Typ`
- Must have exactly these fields:
  - `nym: N`
  - `typ: T`
- Must pass [CSR](#compiler-struct-requirements).
- Must not implement `PartialEq`, `Eq`, or `Hash`.
  - Rationale: every symbol is unique, it is equal only to itself.
- Must not have defining rewrite rules.

### macro sym

- Must pass [macro constructor requirements](#macro-constructor-requirements).
- `name` must be a Rust identifier.
- `sym!(name)` must expand to `let name = $crate::Sym::<$crate::Nym, $crate::Typ>::new_top_rc(stringify!(name));`.
- `sym!(name: typ)` must expand to `let name = $crate::Sym::<$crate::Nym, $crate::Typ>::new_rc(stringify!(name), typ);`.
- `sym!(name: typ; nym)` must expand to `let name = $crate::Sym::<$crate::Nym, $crate::Typ>::new_rc(nym, typ);`.
- Each form must introduce an immutable Rust binding named `name` with type `SymRc` in the invocation's scope.
- A supplied `typ` must implement `Into<Typ>`.
- A supplied `nym` must implement `Into<Nym>`.

## struct Fun

- Must pass [CSR](#compiler-struct-requirements).
- Must have generics:
  - `N`
  - `T`
  - `R`
- Must have exactly these fields:
  - `nym: N`
  - `typ: T`
  - `rules: R`
- `rules` must contain this function's defining rewrite rules.
- Must support zero-argument rules whose bodies have either function or non-function types.
- All rules must have the same number of argument patterns.
- Must not implement a conversion into `Sym` or `SymRc`.
- Registration in a scope must not insert the function into any constructor set.

## enum Binding

- Must pass [CTR](#compiler-type-requirements).
- Must have exactly these variants:
  - `Sym(SymRc)`
  - `Fun(FunRc)`
- `nym()` must return the enclosed symbol's or function's `nym()`.

## struct Scope

- Must pass [CSR](#compiler-struct-requirements).
- Must have generics:
  - `B`, defaulting to `HashMap<String, Binding>`
  - `C`, defaulting to `Constructors`
- Must have exactly these fields:
  - `bindings: B`
  - `constructors: C`
- The default scope must be empty.
- Invariants for the default generic arguments:
  - Each binding key must equal `binding.nym().identifier()`.
  - Symbols and functions must share the same name namespace.
  - `constructors_of(s)` must contain exactly the registered symbols satisfying [Constructor of sym S](#constructor-of-sym-s).
  - Each symbol must occur at most once in each constructor list.
- `Nym::identifier()` must return `&self.short.en.singular.canonical`.
- Registration of an occupied identifier must return an error owning the rejected symbol or function.
- Failed registration must leave the scope unchanged.
- Raw symbol construction must not register the symbol in a scope.
- Binding a function parameter must not register it in the enclosing scope.

### pub fn new_sym

- Must accept `&mut self`, `nym: impl Into<Nym>`, and `typ: impl Into<Typ>`.
- Must return the newly registered symbol as `Result<SymRc, ScopeNewSymError>`.

### pub fn insert_sym

- Must accept `&mut self` and `sym: SymRc`.
- Must register and return that same symbol as `Result<SymRc, ScopeInsertSymError>`.

### pub fn insert_fun

- Must accept `&mut self` and `fun: FunRc`.
- Must register and return that same function as `Result<FunRc, ScopeInsertFunError>`.

### pub fn constructors_of

- Must accept `&self` and `sym: &SymRc`.
- Must return the matching registered symbols as `&[SymRc]`.
- Must return an empty slice when no registered symbol matches.

## type Constructors

- Must be an alias for `HashMap<NonNull<Sym>, Vec<SymRc>>`.
- Keys must use symbol allocation identity.

## enum Typ

- Must pass [CTR](#compiler-type-requirements).
- Must have exactly the following variants:
  - `Top`
  - `One(Exp)`
  - `Fun(DuoVec<SymRc>)`
- Must have methods:
  - `pub fn last(&self) -> &Self`
- `last()` must return the final codomain after all function parameters, or `self` for a non-function type.

### macro typ

- Must pass [macro constructor requirements](#macro-constructor-requirements).
- `typ!()` must expand to `$crate::Typ::top()`.
- `typ!(exp)` must expand to `$crate::Typ::one(exp)`.
- `typ!(param => typ)` must expand to `$crate::Typ::fun(&param, typ)`.
- Each form must be an expression with result type `Typ`.
- A supplied `exp` must implement `Into<Exp>`.
- `param` must be an identifier bound to a value of type `SymRc`.
- A supplied `typ` must implement `Into<Typ>`.

### macro top

- Must pass [macro constructor requirements](#macro-constructor-requirements).
- Must accept only the empty operand list.
- `top!()` must expand to `$crate::Typ::top()`.
- Its expansion must be an expression with result type `Typ`.

## enum Exp

- Must pass [CTR](#compiler-type-requirements).
- Must have exactly the following variants:
  - `Sol(SymRc)`
  - `Fun(FunRc)`
  - `App(ExpBox, ExpBox, TypBox)`
- Must have methods:
  - `pub fn app(fun: impl Into<Exp>, arg: impl Into<Exp>) -> Result<Self, InvalidApplicationError>`

### macro exp

- Must pass [macro constructor requirements](#macro-constructor-requirements).
- Must accept at least one expression operand.
- `exp!(value)` must expand to `$crate::Exp::from(value.clone())`.
- For the one-operand form, `value.clone()` must be a valid expression whose result implements `Into<Exp>`.
- `exp!(fun, arg)` must call `$crate::Exp::app(fun, arg)` exactly once.
- For the two-operand form, `fun` and `arg` must implement `Into<Exp>`.
- If that call returns `Ok(exp)`, the two-operand form must return `exp`.
- If that call returns `Err(error)`, the two-operand form must panic with a message containing `should pass the type check` and the `Debug` representation of `error`.
- For three or more operands, `exp!(first, second, rest...)` must expand recursively to `$crate::exp!($crate::exp!(first, second), rest...)`.
- Operand evaluation must stop if an earlier application panics.
- Each successful form must be an expression with result type `Exp`.

## impl Render for PlainRenderer

- For `render_sym(sym)`, let `name = sym.nym().short.en.singular.to_string()` and `typ = self.render_typ_inner(sym.typ())`.
- `render_sym(sym)` must return `Some(format!("{name} : {typ}"))`.
- A zero-argument rule with rendered name `name` and rendered body `body` must render as `format!("{name} => {body}")`.

## struct DuoVec

- Must be declared as a tuple struct with one private field of type `Vec<T>`.
- Invariant: `self.0.len() >= 2`.
- Must have methods:
  - `pub fn first(&self) -> &T`
  - `pub fn last(&self) -> &T`
- `first()` must return a reference to the first element of `self.0`.
- `last()` must return a reference to the last element of `self.0`.

## .mise/tasks/spec/fix.sh

- Must run sequentially from the repository root:
  1. `cargo build --release --manifest-path spec/Cargo.toml`
  2. `cargo fix --package baselang --lib`
     - Environment:
       - `RUSTC_WORKSPACE_WRAPPER`: absolute path to the built `spec` executable.

## Constructor of type X

- Must be a [sym](#struct-sym).
- Let `result` be the normal form of `sym.typ().last()` under the theory's rewrite rules.
- `result` must be `Typ::One(_)`.
- For `X: &Typ`, `result` must be alpha-equivalent to the normal form of `X`.

Examples:

- `Zero : Nat` is a constructor of type `Nat`
- `Succ : Nat -> Nat` is a constructor of type `Nat`
- `Nil : (T : ^_^) -> List T` a constructor of type `List T`

## Constructor of sym S

- Must be a [sym](#struct-sym).
- The normal form of `sym.typ().last()` must be `Typ::One(exp)`.
- Define `head(Exp::App(fun, _, _)) = head(fun)`.
- Define `head(exp) = exp` for `Exp::Sol(_)` and `Exp::Fun(_)`.
- For `S: &SymRc`, `head(exp)` must be `Exp::Sol(s)` with `Rc::ptr_eq(s, S)`.

Examples:

- `Zero : Nat` is a constructor of sym `Nat`
- `Succ : Nat -> Nat` is a constructor of sym `Nat`
- `Nil : (T : ^_^) -> List T` a constructor of sym `List` (because `List` is the head sym of `List T`) (note that `List T` is a type, not a sym)
- `List : Type -> Type` is not a constructor of any sym.

## Smart application

- Must match the supplied argument to the first parameter whose type unifies with the argument's type.
- Omitted preceding arguments:
  - Type `Typ::Top`: must be inferred by unification.
  - Other types: must be inferred by unification, falling back to canonical values of their inferred types.

Examples:

```baselang
Bool : ^_^
True : Bool
F : (T : ^_^) -> (x : T) -> ^_^

A1 : F Bool True
A2 : F True
```
