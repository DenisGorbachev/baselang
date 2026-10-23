# Baselang language specification

## Purpose

Baselang is a language for resolving disputes between people.

The disputes are resolved by formal logic. Every person expresses their beliefs in Baselang, then asks the compiler to check them. The compiler performs two checks:

- Syntactic (makes sure that definitions are well-formed)
- Semantic (makes sure that definitions don't contradict each other)

The semantic check tries to find a proof of `Bullshit` (a built-in type with no constructors) (aka `False`, `Void`, `Bottom`). If the compiler finds a proof of `Bullshit`, it presents the full proof to the person who wrote it.

## Polysyntax

- Baselang supports multiple input syntaxes for the same language constructs.
- This specification uses explicit arrow syntax.
- Renderers may produce output that cannot be parsed back.

## Canonical notation

- `name : T` declares a name with type `T`.
- `(x : A) -> B` is a function type whose result type may depend on `x`.
- `A -> B` is a function type with an unnamed parameter of type `A`.
- `f a` applies `f` to `a`.
- `lhs => rhs` declares a rewrite rule.
- `name => rhs` defines a name with a zero-argument rewrite rule.
- `=` is not a definition operator.
- `@command args...` invokes a compiler command.
- Application associates to the left: `f a b` means `(f a) b`.
- Function-type arrows associate to the right: `A -> B -> C` means `A -> (B -> C)`.
- Application binds more tightly than arrows.
- Parentheses group expressions.
- In `(x : A) -> B`, `x` is bound in `B`.
- Canonical applications must not insert omitted arguments.
- Partial application is permitted.
- Other input syntaxes may infer omitted arguments.

## Compiler

- The compiler processes the source in a single pass.
- A source file is a sequence of commands.
- Commands execute in source order.
- Names beginning with `@` identify compiler operations.
- Each command specifies the permitted forms of its arguments.
- `@print` writes its string argument followed by a newline.

```baselang
@print "Hello"
```

## Names and bindings

- Names are case-sensitive.
- Qualified names such as `Bool.True` identify declarations within namespaces.
- Redeclaring a name in the same scope is rejected.
- A named declaration must use `Sym`.
- Adding a defining rule does not change the owning declaration's identity or type.
- A type declaration and its defining rules may occur on separate lines.
- A declaration's name must be an identifier.
  - `2 => "Hello"` is invalid.
  - `"H" => "Hello"` is invalid.
- `_` in a parameter position introduces an unnamed binder.

```baselang
text => "Hello"
@print text
@print text
```

## Strings and comments

- Double quotes delimit string literals.
- String literals support the following escapes:
  - `\"` represents a quote.
  - `\\` represents a backslash.
  - `\n` represents a newline.
  - `\r` represents a carriage return.
  - `\t` represents a tab.
- Unknown string escapes are rejected.
- `//` starts a comment extending to the end of the line.
- `/*` and `*/` delimit block comments.
- Block comments may nest.
- Comment delimiters inside strings are ordinary string content.
- Comments separate tokens like whitespace.

## Quoted commands

- Triple backticks delimit a quoted sequence of compiler commands.
- Quoting stores commands without executing them.
- `@evaluate` executes a quoted command block.
- External names retain their bindings from the quotation's lexical scope.
- Declarations produced by evaluation enter the current theory.

````baselang
print_hello_twice => ```
@print "Hello"
@print "Hello"
```
@evaluate print_hello_twice
````

## Terms and types

- Every term has a type annotation.
- `Type` means "unknown type".
- `^_^` is an alternative spelling of `Type` used by some renderers.
- `Type` does not have a type.
- `Type` is not a term.
- Anything can be substituted for a variable of type `Type`.
- `A : Type` and `B : Type` do not establish that `A` and `B` have the same type.
- `A : T` and `B : T` establish that both have the known type `T`.
- Every term can itself be used as a type.
  - `T : Type; A : T; B : A` is permitted.
- No term may occur in its own type or in the types of its ancestors in the type hierarchy.
  - `T : T` is forbidden.
  - `A : B; B : A` is forbidden.
  - `T : T -> Type` is forbidden.
  - `A : B; B : A -> Type` is forbidden.
- There is no universe hierarchy.
- Function types are not terms.
- Function types do not have types.
- Data types and propositions both have type `Type`.

```baselang
List : (T : Type) -> Type
Eq : (T : Type) -> (a : T) -> (b : T) -> Type
```

## Constructors

- The terminal type of `(x : A) -> B` is the terminal type of `B`.
- The terminal type of a non-function type is that type itself.
- The head of a symbol is that symbol.
- The head of an application `f a` is the head of `f`.
- Constructor membership must use the normal form of the terminal type.
- A declared `Sym` is a constructor of that normal form if the normal form is a term and the symbol has no defining rules in the scope under consideration.
- It is a constructor of a `Sym` `S` if that normal form's head is `S`.
- `Type` has no head.
- A symbol whose terminal type is `Type` is not a constructor.
- A scope's constructor sets contain exactly its registered symbols satisfying this definition.
- Parameter binders do not enter the enclosing scope's constructor sets.
- Adding a symbol's first defining rule removes that symbol from all constructor sets in that scope.
- Constructor membership must be recomputed when changes to defining rules change a registered symbol's terminal type's normal form.

### Addition as a relation

```baselang
Nat : Type
Zero : Nat
Step : (n : Nat) -> Nat

Add : (x : Nat) -> (y : Nat) -> (sum : Nat) -> Type
Add.Zero : (n : Nat) -> Add Zero n n
Add.Step : (m : Nat) -> (n : Nat) -> (sum : Nat) -> (proof : Add m n sum) -> Add (Step m) n (Step sum)
```

`Add.Zero` and `Add.Step` are constructors of `Add`.

## Defined symbols

- A defined symbol is a `Sym` with at least one defining rewrite rule in the scope under consideration.
- The scope must store defining rules indexed by their head symbol's allocation identity.
- Simple definitions and definitions by cases must use the same symbol and rule representations.
- A symbol with defining rules must not enter any constructor set in that scope.

### Simple definitions

- `name => rhs` must introduce a `Sym` with a zero-argument rule if `name` is not already declared.
- Its type must be inferred from `rhs` when no type signature is supplied.
- If the type cannot be inferred, an explicit type signature is required.
- References in `rhs` retain their bindings from the definition's lexical scope.
- If `name` is already declared, its existing symbol must be the rule's head.
- Introducing a symbol through a simple definition must register the symbol and its rule atomically.
- An explicit type signature must agree with the type of `rhs` under definitional conversion.
- A recursive definition requires its type signature before its defining rules.
- A simple definition may have a function type.

## Rewrite rules

```baselang
Nat : Type
Zero : Nat
Step : (n : Nat) -> Nat

Add : (x : Nat) -> (y : Nat) -> Nat
Add Zero n => n
Add (Step m) n => Step (Add m n)
```

`Nat`, `Zero`, `Step`, and `Add` are `Sym`s. The scope associates both rules with `Add`. `Zero` and `Step` are constructors of `Nat`; `Add` is a defined symbol.

```baselang
One => Step Zero
Increment => Add One
```

`One` has type `Nat`; `Increment` has type `Nat -> Nat`. Both have a zero-argument defining rule.

### Rule arity

- A rule's arity is the number of argument patterns following its head on the left-hand side.
- All rules associated with one symbol in a scope must have the same arity.
- Rule arity may be zero even when the declared type is a function type.
- With fewer arguments than the rule arity, an application must remain unreduced at its head.
- A matching rule consumes exactly its arity's worth of arguments.
- Any remaining arguments must be applied to the rewritten result.

### Binding and matching

- A rewrite rule has its associated `Sym` as its head.
- Zero-argument definitions occurring inside argument patterns must be unfolded recursively before pattern validation.
- Unfolding must preserve the definition's lexical bindings.
- Expanded argument patterns must consist of applications of symbols without defining rules and pattern variables.
- Any symbol with defining rules remaining in an expanded argument pattern is rejected.
- An argument-pattern identifier that does not resolve to a declared constant introduces a rule-local variable.
- A pattern variable's type is inferred from the function signature and enclosing symbol applications.
- Ambiguous pattern-variable types are rejected.
- Each pattern variable must occur exactly once outside forced constructor arguments on the left-hand side.
- A constructor argument is forced when unifying the constructor's result type with the expected pattern type determines that argument.
- A forced argument must be definitionally equal to the value determined by that unification.
- An occurrence inside a forced argument must not introduce an additional pattern-variable binding.
- `_` in an argument pattern must introduce a fresh anonymous variable.
- An anonymous variable must not be available by name in the right-hand side.
- Pattern variables are bound throughout the right-hand side.
- Patterns match symbol identity and application structure after argument normalization.
- An argument that does not expose the required symbol leaves the application unreduced.
- Pattern type inference, linearity checks, and overlap checks must use the expanded patterns.

```baselang
F : Nat -> Nat
F One => Zero
```

The argument pattern `One` expands to `Step Zero`. Adding `F (Step Zero) => Zero` would overlap the existing rule.

### Admissibility

- Both sides of a rewrite must have the same type under the pattern context.
- Rules for the same function must not overlap.
- Logical reduction must terminate.
- Logical reduction must be confluent.
- Recursive calls must decrease under a well-founded relation.
- The implementation must establish structural termination by size-change analysis of calls between defined symbols.
- Every idempotent cycle in the composed size-change graph must contain a strict decrease on its diagonal.
- Calling or returning a function-valued pattern variable requires termination evidence beyond this structural check.
- The implementation must reject definitions when this check cannot establish termination.
- The entire right-hand side need not be syntactically smaller than the left-hand side.
- A total function with rule arity `k` must have an applicable rule for every well-typed closed tuple of `k` arguments after normalization in the finalized theory.

```baselang
Bool : Type
Bool.True : Bool
Bool.False : Bool
Neg : (b : Bool) -> Bool
Neg Bool.True => Bool.False
Neg Bool.False => Bool.True
```

- Adding `Neg Bool.False => Bool.False` is rejected because it overlaps the existing rule.
- Replacing the second rule with `Neg Bool.False => Zero` is rejected because its result has type `Nat`.

## Theory construction

- The constructor set remains editable until the theory is finalized.
- The rewrite-rule set remains editable until the theory is finalized.
- Constructor classification and argument-pattern validity must use the finalized rewrite-rule set.
- A later defining rule must trigger revalidation of earlier patterns containing its head symbol.
- Coverage checking must refine dependent argument types when splitting an argument into constructor cases.
- Constructor cases whose result types cannot unify with the refined argument type must be excluded.
- A type equation blocked on a defined symbol must not be classified as an impossible constructor case.
- Validation must reject blocked type equations when it cannot establish coverage or non-overlap.
- Theory checks run at the end of the single pass:
  - Typing under the finalized rewrite-rule set.
  - Argument-pattern validity after unfolding zero-argument definitions.
  - Totality.
  - Strict positivity.
  - Termination.
  - Productivity, when coinductive constructs are supported.
  - Rule overlap.
