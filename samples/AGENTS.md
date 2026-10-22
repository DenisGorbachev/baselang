# Baselang language specification

This file is the canonical specification of Baselang. Other documents and `.base` files are supporting material and syntax experiments; disagreements must be resolved in this specification. The specification describes intended language behavior, not the current implementation's completeness. The unresolved design requirements at the end are part of its scope but do not yet define usable primitives.

## Purpose

Baselang is a language for expressing ideas and checking proofs, with the aim of resolving disputes through explicit assumptions and formal logic.

A person states an idea, supplies a proof, and asks the compiler to check it. An LLM may produce the proof, but its output must pass the same checks as a human-written proof. A checked theorem establishes a conclusion relative to its assumptions; it does not establish that those assumptions describe physical reality.

The long-term compatibility goal is to support a port of Lean's Mathlib. This requires a sound account of its mathematical definitions, theorem statements, proofs, and assumptions. Similar surface syntax alone does not establish compatibility.

## Polysyntax and canonical notation

Baselang is polysyntactic: multiple surface syntaxes may express the same language constructs. Each dialect must translate to a common core representation with the same binding, typing, and reduction behavior. Choosing a dialect must not change the logical meaning of a program.

This specification uses the explicit arrow notation:

| Form | Meaning |
| --- | --- |
| `name : T` | Declare a name with type `T`. |
| `name = value` | Bind a new name to an existing value. |
| `(x : A) -> B` | A function type whose result type `B` may depend on `x`. |
| `A -> B` | A function type with an unnamed parameter of type `A`. |
| `f a b` | Apply `f` to `a`, then apply the result to `b`. |
| `lhs => rhs` | A rewrite rule reducing a matched left-hand side to its right-hand side. |
| `@name argument` | Invoke a compiler command. |

- Application associates to the left: `f a b` means `(f a) b`.
- Function-type arrows associate to the right: `A -> B -> C` means `A -> (B -> C)`.
- Application binds more tightly than `->` or `=>`. Parentheses group expressions.
- In `(x : A) -> B`, `x` is bound in `B`, including later parameter types, but not in `A`.
- `->` constructs function types. `=>` declares reduction. A rewrite is not a function type or a proof of propositional equality.
- Canonical applications supply their arguments explicitly. Dialects may infer arguments only when their elaboration produces a fully determined, checked core expression.
- `Type` in older examples must not silently be treated as a synonym for `^_^`: the latter is an unknown-type marker under the current design.

## Compiler commands and checking

A source file contains a sequence of commands. Declarations, bindings, and rewrite rules change the theory being constructed. Commands whose names begin with `@` invoke compiler operations. A command has a name and zero or more parameters; each command specifies the permitted forms of its parameters.

For example:

```baselang
@print "Hello"
```

`@print` writes its string argument followed by a newline. Output does not create a declaration or prove a proposition.

The compiler distinguishes:

1. Parsing: the source conforms to the selected dialect's grammar.
2. Elaboration: names, bindings, and inferred arguments resolve to core constructs.
3. Checking: types, proofs, declarations, and rewrites satisfy the logical rules in a finalized theory.
4. Optional proof search: the compiler attempts to construct additional proofs, including proofs of contradiction.

Parsing and executing commands successfully do not by themselves certify a theorem. Syntax extensions, generated code, and proof-search results must pass the same logical checker. Adding a display command must not add a logical axiom.

The contradiction target is `Bullshit`, a distinguished empty type with no constructors or defining rewrites. `False`, `Void`, and `Bottom` may be aliases of this type; they must not be confused with a Boolean constructor such as `Bool.False`. The compiler must reject attempts to extend this built-in type with constructors or defining rewrites.

If proof search constructs a term of type `Bullshit` from the theory's assumptions, the compiler reports the checked proof and its assumptions. Failure to find such a term, including exhaustion of a search budget, means that no contradiction was found; it is not a certificate of consistency. Proof search is not a prerequisite for checking a supplied proof.

## Names, bindings, and comments

Names are case-sensitive. Qualified names such as `Bool.True` identify declarations within namespaces. A declaration's identity is determined by resolution in its scope, not by its printed spelling alone. A binder has a fresh identity; substitution must preserve free variables and avoid capture. Redeclaring a name in the same scope is rejected.

The forms below use `$name`, `$type`, and `$value` as specification placeholders, not literal identifiers:

- `$name : $type` declares a name and its type without supplying a defining value.
- `$name = $value` creates an immutable binding to an existing value, rather than creating another constructor or asserting propositional equality.
- `$name` must be an identifier. Numeric and string literals are not valid binding names, so `2 = "Hello"` and `"H" = "Hello"` are rejected.
- `_` in an unnamed parameter position introduces a fresh binder that cannot be referenced by name. It does not supply a missing proof or missing argument.

For example:

```baselang
text = "Hello"
@print text
@print text
```

The output is two lines containing `Hello`. Reusing the binding does not consume it. An alias preserves the identity and meaning of its value.

String literals are delimited by double quotes. Their contents are text, not executable commands. The escapes `\"`, `\\`, `\n`, `\r`, and `\t` represent a quote, a backslash, a newline, a carriage return, and a tab; unknown escapes are rejected.

- `//` starts a comment extending to the end of the line.
- `/*` starts a comment ending at the matching `*/`; block comments may nest.
- Comment delimiters inside strings are ordinary string content.
- Comments do not introduce declarations or change the meaning of adjacent tokens; they separate tokens like whitespace.

## Quoted commands and evaluation

A code block delimited by triple backticks is a quoted sequence of compiler commands. Quoting stores the commands without executing them. `@evaluate` executes a quoted command block.

````baselang
print_hello_twice = ```
@print "Hello"
@print "Hello"
```
@evaluate print_hello_twice
````

The binding alone produces no output. Evaluating it prints two lines containing `Hello`.

A quoted block is a compiler-level value. It is not automatically a logical proof or a function term. Names referring outside the block retain their lexical bindings from the quotation site. Declarations produced by evaluation enter the current theory and must be validated before that theory is certified. The evaluator must preserve binder identities when expanding a block.

The concrete representation of quoted syntax, parameterized compiler commands, nested quotations, and antiquotation remains to be specified. No command may manufacture a trusted theorem by bypassing checking.

## Terms and types

- Every term has a type annotation.
- Some terms have the special annotation `^_^`, which means that the type is unknown.
  - `^_^` itself does not have a type and is not a term.
  - `A : ^_^` and `B : ^_^` do not establish that `A` and `B` have the same type.
  - `A : T` and `B : T` establish that both have the known type `T`; they do not establish that `A` and `B` are equal.
  - An unknown type is not permission to bypass checking an application or a rewrite.
- Every term can itself be used as a type: `T : ^_^; A : T; B : A` is permitted.
- No term may occur in its own type or in the types of its ancestors in the type hierarchy.
  - `T : T` is forbidden.
  - `A : B; B : A` is forbidden.
  - `T : T -> ^_^` is forbidden.
  - `A : B; B : A -> ^_^` is forbidden.
- There is no universe hierarchy in the current design; the type hierarchy is intended to replace it.
- Function types are types but are not themselves terms and do not have types in the current design.
- Data types and propositions both use `^_^`; there is no separate `Type` and `Prop` classification.

Examples of the intended polymorphic declarations are:

```baselang
List : (T : ^_^) -> ^_^
Eq : (T : ^_^) -> (a : T) -> (b : T) -> ^_^
```

These declarations do not yet specify the constructors or elimination principles of `List` and `Eq`. In particular, declaring `Eq` is not sufficient to define equality.

The interaction between unknown-type quantification, terms used as types, and the exclusion of function types from terms requires formal formation and substitution rules. Acyclicity alone is not a specified consistency argument. The rules above must not be read as an implicit coercion from a term to every ancestor in its type hierarchy.

## Constructors and natural numbers

Constructors introduce the possible forms of data. Bindings provide names for existing forms. For natural numbers:

```baselang
Nat : ^_^
Zero : Nat
Step : (n : Nat) -> Nat

One = Step Zero
Two = Step One
Three = Step Two
```

`Zero` and `Step` are the constructors in this example. `One`, `Two`, and `Three` are aliases of constructed values. `Step` is a constructor with a function type; it need not be an effectful process.

The distinction between introducing an inductive type, adding one of its constructors, postulating an assumption, and declaring a function awaiting rewrites requires explicit declaration categories. The examples above identify their categories in prose; `name : T` alone does not yet encode those categories.

Numeric literals require a separately specified elaboration into constructors or another checked representation. They are not introduced by rebinding literal tokens.

## Rewrite rules

Functions may be defined by directed rewrite rules. The canonical addition example is:

```baselang
Add : (x : Nat) -> (y : Nat) -> Nat
Add Zero n => n
Add (Step m) n => Step (Add m n)
```

The first rule reduces addition with `Zero` as its first argument. The second reduces addition with `Step m` as its first argument to a recursive call on `m`.

### Binding and matching

- A rewrite rule has a declared function as its head and supplies all parameters of that declaration on the left-hand side.
- Argument patterns consist of constructor applications and pattern variables. Constructors are resolved by declaration identity, including through aliases.
- Within an argument pattern, an identifier that does not resolve to a declared constant introduces a rule-local pattern variable. Its type must be uniquely determined by the function signature and enclosing constructor patterns, or elaboration fails.
- The pattern fragment defined here is left-linear: each pattern variable occurs exactly once on the left-hand side. Its binding is in scope throughout the right-hand side. The same spelling in another rule introduces a different variable. Dependent patterns with forced parameters or indices require an extension specified below.
- Every free name on the right-hand side must resolve to a pattern variable or an existing declaration. Right-hand sides cannot introduce unchecked metavariables.
- A matching substitution is applied without variable capture. Matching a constructor pattern inspects constructor structure, rather than searching for arbitrary proofs of equality.
- Open arguments that do not expose the required constructor leave an application unreduced; this does not by itself make an otherwise well-typed term invalid.

In the addition rules, `Zero` and `Step` are constructors, while `m` and `n` are rule-local variables of type `Nat`. For example, `Add (Step Zero) (Step Zero)` reduces to `Step (Step Zero)`.

### Admissibility

- Both sides must have the same type under the pattern context and every permitted substitution. For dependent arguments, checking must account for the refinements introduced by the patterns.
- Distinct rules for the same function must not overlap: no well-typed application may match both left-hand sides. Source order does not choose between overlapping rules.
- Constructors cannot receive defining rewrite rules. Arbitrary proven equalities cannot be installed as computation rules without satisfying the rewrite checks.
- The combined reduction system must be terminating and confluent on well-typed logical terms. The compiler must establish these properties before using the rules for certified conversion.
- Recursive definitions need a well-founded decrease in recursive calls. The entire right-hand side need not be syntactically smaller than the left-hand side: the addition rule is justified by replacing `Step m` with its subterm `m` in the recursive call.
- A function presented as total must cover every possible constructor case for its declared domain in the finalized theory. Any support for partial computations must be kept outside logical conversion unless an appropriate logical encoding is provided.

For example:

```baselang
Bool : ^_^
Bool.True : Bool
Bool.False : Bool
Neg : (b : Bool) -> Bool
Neg Bool.True => Bool.False
Neg Bool.False => Bool.True
```

Adding `Neg Bool.False => Bool.False` is rejected because it overlaps the existing rule. Replacing the second rule with `Neg Bool.False => Zero` is rejected because its result has type `Nat`, not `Bool`.

The exact dependent-pattern calculus, conversion rules, and admissible termination evidence remain unresolved. The requirements above are obligations on that design, not an assertion that arbitrary user-written rewrite systems are decidable.

## Theory construction and finalization

Constructors and rewrite rules may be added or removed while constructing a theory. This changes the candidate theory; it does not mutate an already certified theory.

After reading the candidate declarations, the compiler must check the applicable totality, strict positivity, termination, productivity, and overlap requirements. A theory that has not passed its required checks must not export certified theorems. Productivity checks apply only if a coinductive construct is specified; they do not permit nonterminating logical conversion.

Certification is relative to a fixed theory snapshot, including its assumptions and dependencies. Adding or removing constructors or rules invalidates the affected certification and requires checking again. Previously derived induction principles and coverage results must not be reused as if the theory were unchanged. The built-in empty type remains sealed.

## Unresolved requirements for a Mathlib port

The following are specification requirements to resolve before claiming that Mathlib can be translated with its proofs preserved. They do not require implementing the port now. A feature may be supplied as a core primitive or through a justified encoding whose typing and computation rules are specified.

| Area | Required decision or missing primitive |
| --- | --- |
| Type formation and size | Give judgments for well-formed types, contexts, terms, and substitution, including the exact meaning of quantification over `^_^`. Explain how Lean's universe-polymorphic declarations and level constraints are represented without collapsing distinctions needed for soundness. |
| Functions as mathematical objects | Specify dependent function introduction, application, abstraction, local bindings, and computation. Resolve how a generic construction can take a function type as an argument when function types are not terms; `List (Nat -> Nat)` is a concrete missing case. |
| Equality and conversion | Define propositional equality, reflexivity, and dependent transport or an equivalent eliminator. Specify definitional conversion, including the treatment of aliases, binders, function computation, and extensionality. Term rewriting alone does not provide all equality reasoning. |
| Inductive definitions | Distinguish constructors from assumptions and defined functions. Specify parameters, indices, strict positivity, dependent eliminators, their computation rules, and mutual or nested definitions. Give a precise acceptance criterion for recursive definitions, including well-founded recursion. |
| Dependent rewriting | Extend the constructor-pattern fragment with checked forced parameters and indices, or translate dependent matching to eliminators. Specify normalization during matching, impossible cases, overlap checking after index refinement, and admissible evidence for termination and confluence. |
| Propositions | Explain how the unified data/proposition design represents Lean's impredicative `Prop`, definitional proof irrelevance, and elimination restrictions. Applying proof irrelevance indiscriminately to data would identify distinct data values. |
| Quotients and classical reasoning | Specify quotient formation, introduction, elimination, soundness, and computation, or a sufficient encoding. State the status of propositional extensionality, function extensionality, and choice, including which are assumptions and which are derived. Distinguish noncomputable definitions from executable computations. |
| Trusted assumptions | Give explicit syntax and checking rules for postulates, proved theorems, opaque definitions, and unresolved obligations. Record transitive axiom dependencies; incomplete proofs must never silently become certified theorems. |
| Elaboration and modules | Specify the canonical grammar, declaration and command boundaries, namespaces, imports, name visibility, and dialect selection. Define how inference, overloaded notation, coercions, and typeclass-style instance arguments become explicit checked terms. Specify the remaining quotation and command-extension forms. |
| Proof interchange | Define an eventual translation contract for declarations and proof terms, with fixed dependency versions and preserved theorem statements and axiom dependencies. Successful Lean elaboration must not substitute for checking the translated result. Translating tactic implementations is a separate question from translating the proof terms they produce. |

The compatibility reference is Lean's [type system](https://lean-lang.org/doc/reference/latest/The-Type-System/), including [functions](https://lean-lang.org/doc/reference/latest/The-Type-System/Functions/), [universes](https://lean-lang.org/doc/reference/latest/The-Type-System/Universes/), [inductive types](https://lean-lang.org/doc/reference/latest/The-Type-System/Inductive-Types/), [propositions](https://lean-lang.org/doc/reference/latest/The-Type-System/Propositions/), [quotients](https://lean-lang.org/doc/reference/latest/The-Type-System/Quotients/), and [axioms](https://lean-lang.org/doc/reference/latest/Axioms/). These references describe the source system; they do not establish that Baselang's current foundations already implement or soundly encode it.
