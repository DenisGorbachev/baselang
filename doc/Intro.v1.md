Baselang is a language for finding contradictions.

Baselang fights AI slop by detecting contradictions in LLM output. The output in natural language is inherently ambiguous, so if Baselang doesn't detect any contradictions, that doesn't mean that the text is free of them. However, if Baselang **does** detect contradictions, then it **guarantees** that the text is inconsistent (in at least one specific interpretation).

Baselang fights human slop, too. If you disagree with a human, you can use Baselang to find contradictions in their logic. If Baselang finds a contradiction, you can present it as an argument. However, Baselang may misunderstand their logic. In this case, they can clarify it with more precise definitions. The clarification process often brings new ideas that become the foundation of a mutual agreement.

## Type theory

Baselang is based on a novel flavor of a type theory. It's similar to Lean with one important difference:

* In Lean, you can only define types and constructors (2 levels max):
  * `Animal : Type`
  * `Cat : Animal`
  * (that's it, no more levels)
* In Baselang, you can define types, constructors, constructors of constructors, etc (no limits):
  * `Animal : Type`
  * `Cat : Animal`
  * `Garfield : Cat`

That means you can define deep hierarchies, but that's only a part of the difference. The other part is that you can pass `Garfield` as argument to any function that expects `x : Animal` (because `Garfield : Cat` and `Cat : Animal`). In general, a lower-level term is accepted in a place where a higher-level term is expected.

Notes:

* The hierarchy can't have cycles (so `A : B` and `B : A` is invalid)
* The type can be closed to prevent adding constructors (so `Empty : Type` can be truly empty)

Technical details:

When you pass `Garfield` to a function that expects `x : Animal`, the compiler makes a copy of the function and replaces `Animal` with `Cat`. Sometimes the replacement leads to a logical error. If this happens, the compiler emits the error and rejects the definition. You can fix the error by explicitly writing `Garfield <:> Animal`, which converts it to `Cat : Animal` (this allows the compiler to preserve the original function with `x : Animal` without replacement).

## Rust

Baselang is written in Rust. Baselang is also written _with_ Rust, because that's the easiest way to implement metaprogramming.

For example, here's a definition of natural numbers:

```rust
// TODO
```

And here's the newtype pattern (with a touch of metaprogramming):

```rust
// TODO
```

## Autoformalization

Autoformalization is the process of translating a text from informal into formal language (e.g. from English into Baselang). 
