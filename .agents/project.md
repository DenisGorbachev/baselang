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
- Defining rewrite rules must be stored in the owning scope.
- Adding a defining rewrite rule must preserve the symbol's allocation identity and type.

### macro sym

- Must pass [macro constructor requirements](#macro-constructor-requirements).
- `name` must be a Rust identifier.
- `sym!(name)` must expand to `let name = $crate::Sym::<$crate::Nym, $crate::Typ>::new_top_rc(stringify!(name));`.
- `sym!(name: typ)` must expand to `let name = $crate::Sym::<$crate::Nym, $crate::Typ>::new_rc(stringify!(name), typ);`.
- `sym!(name: typ; nym)` must expand to `let name = $crate::Sym::<$crate::Nym, $crate::Typ>::new_rc(nym, typ);`.
- Each form must introduce an immutable Rust binding named `name` with type `SymRc` in the invocation's scope.
- A supplied `typ` must implement `Into<Typ>`.
- A supplied `nym` must implement `Into<Nym>`.

## struct Rewrite

- Must pass [CSR](#compiler-struct-requirements).
- Must have generics:
  - `P`, defaulting to `Vec<Exp>`
  - `E`, defaulting to `Exp`
- Must have exactly these fields:
  - `patterns: P`
  - `body: E`
- For the default generic arguments, `patterns` must contain the ordered argument patterns after the rule's head.
- An empty `patterns` must specify a zero-argument rule.
- A zero-argument rule's body may have a function or non-function type.

## struct Scope

- Must pass [CSR](#compiler-struct-requirements).
- Must have generics:
  - `B`, defaulting to `HashMap<String, SymRc>`
  - `R`, defaulting to `Rewrites`
  - `C`, defaulting to `Constructors`
- Must have exactly these fields:
  - `bindings: B`
  - `rewrites: R`
  - `constructors: C`
- The default scope must be empty.
- Invariants for the default generic arguments:
  - Each binding key must equal its symbol's `nym().identifier()`.
  - Every rewrite-map key must identify a registered symbol.
  - All rules associated with one symbol must have the same number of argument patterns.
  - Each rule's body must have the type of its head applied to its argument patterns under the rule's pattern context and definitional conversion.
  - `constructors_of(s)` must contain exactly the registered symbols satisfying [Constructor of sym S](#constructor-of-sym-s) in this scope.
  - Each symbol must occur at most once in each constructor list.
- `Nym::identifier()` must return `&self.short.en.singular.canonical`.
- Registration of an occupied identifier must return an error owning the rejected symbol.
- Failed symbol registration or rule insertion must leave the scope unchanged.
- Constructor indexes must reflect changes to defining rules and to the normal forms of registered symbols' terminal types.
- Raw symbol construction must not register the symbol in a scope.
- Binding a function parameter must not register it in the enclosing scope.
- A registered identifier must consist of one or more dot-separated components.
- Each identifier component must begin with an alphabetic character or `_`.
- Subsequent component characters must be alphanumeric characters or `_`.

### pub fn new_sym

- Must accept `&mut self`, `nym: impl Into<Nym>`, and `typ: impl Into<Typ>`.
- Must return the newly registered symbol as `Result<SymRc, ScopeNewSymError>`.

### pub fn insert_sym

- Must accept `&mut self` and `sym: SymRc`.
- Must register and return that same symbol as `Result<SymRc, ScopeInsertSymError>`.

### pub fn insert_rewrite

- Must accept `&mut self`, `sym: SymRc`, and `rewrite: Rewrite`.
- Must return `Result<(), ScopeInsertRewriteError>`.
- Must reject a `sym` whose allocation identity does not match a registered symbol.
- On success, must append `rewrite` to the rules associated with `sym`.
- On error, the returned error must own the rejected `sym` and `rewrite`.
- Before committing an insertion, must validate the new rule's bindings, patterns, cached expression types, and result type.
- Coverage, overlap, and termination checks must use the complete rule set during theory construction.

### pub fn define

- Must accept `&mut self`, `nym: impl Into<Nym>`, and `body: impl Into<Exp>`.
- Must infer the symbol's type from `body`.
- Must register the symbol and its zero-argument rule atomically.
- Must return `Result<SymRc, ScopeDefineError>`.

### pub fn apply

- Must accept `&self`, `fun: impl Into<Exp>`, and `arg: impl Into<Exp>`.
- Must compare the function parameter's type and the argument's type after normalization in this scope.
- Must substitute the argument into the function's result type.
- Must return `Result<Exp, ScopeApplyError>`.

### pub fn normalize

- Must accept `&self` and `exp: &Exp`.
- Must return `Result<Exp, ScopeNormalizeError>`.
- Must normalize application arguments before matching rewrite rules.
- Must substitute all pattern-variable bindings simultaneously into a matching rule's body.
- Must recursively normalize the substituted body.
- Must leave an application with fewer arguments than the head's rule arity unreduced at that head.
- Must apply arguments beyond the rule arity to the rewritten result.
- Must return an error when dynamic size-change checking detects a recursive cycle without structural descent.

### pub fn rewrites_of

- Must accept `&self` and `sym: &SymRc`.
- Must return the rules associated with that symbol's allocation identity as `&[Rewrite]`.
- Must return an empty slice when no rules are associated with that identity.

### pub fn constructors_of

- Must accept `&self` and `sym: &SymRc`.
- Must return the matching registered symbols as `&[SymRc]`.
- Must return an empty slice when no registered symbol matches.

## type Rewrites

- Must be an alias for `HashMap<NonNull<Sym>, Vec<Rewrite>>`.
- Keys must use symbol allocation identity.

## struct Theory

- Must pass [CSR](#compiler-struct-requirements).
- Must have one generic parameter `S`, defaulting to `Scope`.
- Must have exactly one field `scope: S`.
- `Theory::new(scope: Scope)` must return `Result<Self, TheoryNewError>`.
- Construction must validate all declarations and rules against the supplied scope's complete rule set.
- Validation must check bindings, cached expression types, pattern validity, rule result types, overlap, structural termination, and constructor-pattern coverage.
- Successful construction must establish all of these validation predicates as invariants.
- Failed construction must return an error owning the rejected scope.

## trait Parse

- Must have associated type `Error`.
- Must have `fn parse(&mut self, input: &str) -> Result<Theory, Self::Error>`.
- The returned theory must retain all parsed declarations and rewrite rules.
- Final validation must complete before returning `Ok`.

## struct PlainParser

- `parse_scope` must return a provisional scope without requiring complete coverage or termination evidence.
- `expression` must resolve names in its supplied scope and use that scope's application checking.
- A parse error must preserve the source text and identify the failing source line when parsing declarations or rules.

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
- Must have no defining rewrite rules in the scope under consideration.
- Let `result` be the normal form of `sym.typ().last()` under that scope's rewrite rules.
- `result` must be `Typ::One(_)`.
- For `X: &Typ`, `result` must be alpha-equivalent to the normal form of `X`.

Examples:

- `Zero : Nat` is a constructor of type `Nat`
- `Succ : Nat -> Nat` is a constructor of type `Nat`
- `Nil : (T : ^_^) -> List T` a constructor of type `List T`

## Constructor of sym S

- Must be a [sym](#struct-sym).
- Must have no defining rewrite rules in the scope under consideration.
- The normal form of `sym.typ().last()` under that scope's rewrite rules must be `Typ::One(exp)`.
- Define `head(Exp::App(fun, _, _)) = head(fun)`.
- Define `head(Exp::Sol(s)) = s`.
- For `S: &SymRc`, `Rc::ptr_eq(head(exp), S)` must hold.

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
