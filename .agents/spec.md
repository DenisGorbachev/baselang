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
- `name = value` binds a new name to an existing value.
- `(x : A) -> B` is a function type whose result type may depend on `x`.
- `A -> B` is a function type with an unnamed parameter of type `A`.
- `f a` applies `f` to `a`.
- `lhs => rhs` declares a rewrite rule.
- `@command args...` invokes a compiler command.
- Application associates to the left: `f a b` means `(f a) b`.
- Function-type arrows associate to the right: `A -> B -> C` means `A -> (B -> C)`.
- Application binds more tightly than arrows.
- Parentheses group expressions.
- In `(x : A) -> B`, `x` is bound in `B`.
- Canonical applications supply every argument.
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
- A binding created by `=` is immutable.
- A binding name must be an identifier.
  - `2 = "Hello"` is invalid.
  - `"H" = "Hello"` is invalid.
- `_` in a parameter position introduces an unnamed binder.

```baselang
text = "Hello"
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
print_hello_twice = ```
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

## Rewrite rules

```baselang
Nat : Type
Zero : Nat
Step : (n : Nat) -> Nat

Add : (x : Nat) -> (y : Nat) -> Nat
Add Zero n => n
Add (Step m) n => Step (Add m n)
```

### Binding and matching

- A rewrite rule has a declared function as its head.
- The left-hand side supplies every parameter of that function.
- Argument patterns consist of constructor applications and pattern variables.
- Constructor aliases resolve to the underlying constructors.
- An argument-pattern identifier that does not resolve to a declared constant introduces a rule-local variable.
- A pattern variable's type is inferred from the function signature and enclosing constructor patterns.
- Ambiguous pattern-variable types are rejected.
- Each pattern variable occurs exactly once on the left-hand side.
- Pattern variables are bound throughout the right-hand side.
- Constructor patterns match constructor structure.
- An argument that does not expose the required constructor leaves the application unreduced.

### Admissibility

- Both sides of a rewrite must have the same type under the pattern context.
- Rules for the same function must not overlap.
- Constructors cannot receive defining rewrite rules.
- Logical reduction must terminate.
- Logical reduction must be confluent.
- Recursive calls must decrease under a well-founded relation.
- The entire right-hand side need not be syntactically smaller than the left-hand side.
- A total function must cover every constructor case of its domain in the finalized theory.

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
- Theory checks run at the end of the single pass:
  - Totality.
  - Strict positivity.
  - Termination.
  - Productivity, when coinductive constructs are supported.
  - Rule overlap.
