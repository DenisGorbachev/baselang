# Baselang concepts

## Cargo.toml

- All dependency versions and sources must be declared in `workspace.dependencies`.
- Package dependency declarations must use `workspace = true`.
- Package-specific features may be selected in package dependency declarations.
- `workspace.dependencies` must contain:
  - `derive_getters`

## Compiler type requirements

- For a type named `X`, must declare a public type alias named `XRc` whose target is `Rc<X>`.
- `XRc` must declare the same generic parameters, in the same order and with the same defaults, as `X`.
- Each generic argument supplied to `XRc` must be forwarded to the corresponding parameter of `X`.
- Must state each invariant as a predicate over its fields or the results of specified methods.
- Every constructor, conversion, and deserializer that returns a value of the type must establish all declared invariants before returning that value.
- Every public mutator must preserve all declared invariants when called on a value satisfying them.
- Preservation of invariants must hold on both successful and error returns from a mutator.
- A public field or an API exposing mutable access to a field is permitted only if every assignment or mutation available through that access preserves all declared invariants.

Alias: CTR.

## Compiler struct requirements

- Must be a struct.
- Must pass [CTR](#compiler-type-requirements).
- The declared type of each field must be a type parameter of the struct.
- Different fields must use different type parameters in their declarations.
- Must derive `Getters`.

Alias: CSR.

## Macro constructor

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

- Must have exactly these fields:
  - `nym: Nym`
  - `typ: Typ`
  - `constructors: Constructors`
- Must pass [CSR](#compiler-struct-requirements), except its field types must be the concrete types specified above instead of type parameters.
- Must not implement `PartialEq`, `Eq`, or `Hash`.
- Must have [sym! macro constructor](#macro-constructor).

### macro sym

- `name` must be a Rust identifier.
- `sym!(name)` must expand to `let name = $crate::Sym::new_top_rc(stringify!(name), None);`.
- `sym!(name: typ)` must expand to `let name = $crate::Sym::new_rc(stringify!(name), typ, None);`.
- `sym!(name: typ; nym)` must expand to `let name = $crate::Sym::new_rc(nym, typ, None);`.
- Each form must introduce an immutable Rust binding named `name` with type `SymRc` in the invocation's scope.
- A supplied `typ` must implement `Into<Typ>`.
- A supplied `nym` must implement `Into<Nym>`.

## struct Def

- Must have generics:
  - `N`, defaulting to `Nym`
  - `E`, defaulting to `Exp`
- Must have exactly these fields:
  - `nym: N`
  - `exp: E`
- Must pass [CSR](#compiler-struct-requirements).
- `Def<N, Exp>` must have `pub fn typ(&self) -> &Typ` that returns `self.exp.typ()`.
- `From<Def<N, Exp>> for Exp` must return the owned `exp` field.
- `From<&Def<N, Exp>> for Exp` must return `exp.clone()`.
- `From<Def<N, Exp>> for Typ` must return `Typ::One(Exp::from(def))`.
- `From<&Def<N, Exp>> for Typ` must return `Typ::One(Exp::from(def))`.
- For `A: Into<Exp>`, `<Def<N, Exp> as Of<A>>::of(&self, arg)` must return the result of `self.exp.of(arg)`.
- Must not implement a conversion from `Def<N, Exp>` into `Sym` or `SymRc`.
- Must have [def! macro constructor](#macro-constructor).

### macro def

- `name` must be a Rust identifier.
- `def!(name = exp)` must expand to `let name = $crate::Def::<$crate::Nym, $crate::Exp>::new(stringify!(name), exp);`.
- `def!(name = exp; nym)` must expand to `let name = $crate::Def::<$crate::Nym, $crate::Exp>::new(nym, exp);`.
- Each form must introduce an immutable Rust binding named `name` with type `Def<Nym, Exp>` in the invocation's scope.
- A supplied `exp` must implement `Into<Exp>`.
- A supplied `nym` must implement `Into<Nym>`.

## type Constructors

- Must be an alias for `Option<Vec<SymRc>>`.

## enum Typ

- Must pass [CTR](#compiler-type-requirements).
- Must have exactly the following variants:
  - `Top`
  - `One(Exp)`
  - `Fun(DuoVec<SymRc>)`
- Must have methods:
  - `pub fn last(&self) -> &Self`
- Must have [typ! macro constructor](#macro-constructor).
- Must have [top! macro constructor](#macro-constructor).

### macro typ

- `typ!()` must expand to `$crate::Typ::top()`.
- `typ!(exp)` must expand to `$crate::Typ::one(exp)`.
- `typ!(param => typ)` must expand to `$crate::Typ::fun(&param, typ)`.
- Each form must be an expression with result type `Typ`.
- A supplied `exp` must implement `Into<Exp>`.
- `param` must be an identifier bound to a value of type `SymRc`.
- A supplied `typ` must implement `Into<Typ>`.

### macro top

- Must accept only the empty operand list.
- `top!()` must expand to `$crate::Typ::top()`.
- Its expansion must be an expression with result type `Typ`.

## enum Exp

- Must pass [CTR](#compiler-type-requirements).
- Must have exactly the following variants:
  - `Sol(SymRc)`
  - `App(ExpBox, ExpBox, TypBox)`
- Must have methods:
  - `pub fn app(fun: impl Into<Exp>, arg: impl Into<Exp>) -> Result<Self, InvalidApplicationError>`
- Must have [exp! macro constructor](#macro-constructor).

### macro exp

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
- For `render_def(def)`, let `name = def.nym().short.en.singular.to_string()` and `exp = self.render_exp_inner(def.exp(), false, false)`.
- `render_def(def)` must return `Some(format!("{name} = {exp}"))`.

## struct DuoVec

- Must be declared as a tuple struct with one private field of type `Vec<T>`.
- Invariant: `self.0.len() >= 2`.
- Must have methods:
  - `pub fn first(&self) -> &T`
  - `pub fn last(&self) -> &T`
- `first()` must return a reference to the first element of `self.0`.
- `last()` must return a reference to the last element of `self.0`.

## .mise/tasks/spec/fix.sh

- Must execute `cargo build --release --manifest-path spec/Cargo.toml` with the repository root as its working directory.
- Must execute `cargo fix --package baselang --lib` with the repository root as its working directory after the build command succeeds.
- Must set `RUSTC_WORKSPACE_WRAPPER` for the `cargo fix` process to the absolute path of the `spec` executable produced by the build command.
- Must not execute `cargo fix` if the build command fails.
- If either command fails, must exit with a nonzero status.

## Constructor of type X

- Must be a [sym](#struct-sym).
- For `X: &Typ`, must satisfy `sym.typ().last().alpha_eq(X)`.

Examples:

- `Zero : Nat` is a constructor of type `Nat`
- `Succ : Nat -> Nat` is a constructor of type `Nat`
- `Nil : (T : ^_^) -> List T` a constructor of type `List T`

## Constructor of sym S

- Must be a [sym](#struct-sym).
- For `S: &SymRc`, `sym.typ().last()` must be `Typ::One(exp)`.
- Define `head(Exp::Sol(s)) = s`.
- Define `head(Exp::App(fun, _, _)) = head(fun)`.
- Must satisfy `Rc::ptr_eq(head(exp), S)`.

Examples:

- `Zero : Nat` is a constructor of sym `Nat`
- `Succ : Nat -> Nat` is a constructor of sym `Nat`
- `Nil : (T : ^_^) -> List T` a constructor of sym `List` (because `List` is the head sym of `List T`) (note that `List T` is a type, not a sym)

## Smart application

- Must accept an ordered parameter sequence, an argument expression, a unification operation, and a lookup operation for canonical values indexed by type.
- The unification operation must return either a substitution satisfying its input constraints or a failure.
- Must select the lowest parameter index whose type unifies with the argument's type.
- Must assign the argument to the selected parameter.
- Must apply the resulting substitution to the types of all remaining parameters.
- Must process parameters preceding the selected parameter in decreasing index order.
- If the current substitution assigns an expression to a preceding parameter, must use that expression.
- If a preceding parameter has type `Typ::Top` and the substitution does not assign it an expression, must report failure.
- If a preceding parameter has a type other than `Typ::Top` and the substitution does not assign it an expression, must request a canonical value for that substituted type.
- Must report failure if the canonical-value lookup returns no value.
- Must validate every inserted argument using the same application check as an explicitly supplied argument.
- Must report failure if no parameter type unifies with the supplied argument's type.

Examples:

```baselang
Bool : ^_^
True : Bool
F : (T : ^_^) -> (x : T) -> ^_^

A1 : F Bool True
A2 : F True
```
