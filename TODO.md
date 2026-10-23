# TODO

* Add intro
  * Continuous change
    * Our experience is continuous - some things don't change. But the same experience tells us that other things change. This duality gives rise to the idea of "continuous change" - a transformation that maintains some properties while changing the others. And the most fundamental continuity is the continuity itself - the fact that "some things don't change" doesn't change.
* Update positioning
  * Most people don't have a problem with making decisions, but they do have a problem with persuading other people about correctness of their decisions.
* Process knowledge
  * If you can't detect the cycles in the sequence, then it's not valid to use the axiom of the cycle, because the current sequence may be only a part of the cycle

## Language specification

* Maintain the canonical specification in [.agents/spec.md](.agents/spec.md).
* FUN: Specify functions as mathematical objects.
  * Define dependent function introduction.
  * Define function abstraction.
  * Define function application.
  * Define local bindings.
  * Define function computation.
  * Specify how non-term types participate in applications such as `List (Nat -> Nat)` under unrestricted `Type` substitution.
* DCL: Distinguish declaration categories.
  * Specify inductive type declarations.
  * Specify constructor declarations.
  * Specify function declarations.
  * Specify postulates.
  * Specify proved theorems.
  * Specify opaque definitions.
  * Specify unresolved proof obligations.
  * Record transitive axiom dependencies.
* IND: Specify inductive definitions.
  * Define parameters.
  * Define indices.
  * Define strict positivity.
  * Define dependent eliminators.
  * Define elimination from the empty type.
  * Define eliminator computation rules.
  * Define mutual inductive declarations.
  * Define nested inductive declarations.
  * Specify the acceptance criterion for well-founded recursion.
* EQV: Specify equality.
  * Define reflexivity.
  * Define dependent transport.
  * Specify definitional conversion.
  * Specify alias unfolding.
  * Specify conversion under binders.
  * Specify function extensionality.
* RWT: Extend dependent rewriting.
  * Specify forced parameter patterns.
  * Specify forced index patterns.
  * Specify normalization during matching.
  * Specify impossible cases.
  * Specify overlap checking after index refinement.
  * Specify admissible termination evidence.
  * Specify admissible confluence evidence.
  * Consider translating dependent matches into eliminators.
* SYN: Specify input syntax.
  * Define the canonical grammar.
  * Define command boundaries.
  * Define namespace resolution.
  * Define imports.
  * Define name visibility.
  * Define input dialect selection.
  * Define argument inference.
  * Define overloaded notation.
  * Define coercions.
  * Define typeclass-style instance arguments.
  * Define numeric literals.
* CMD: Specify compiler metaprogramming.
  * Define syntax values.
  * Define parameterized commands.
  * Define nested quotation.
  * Define antiquotation.
  * Define command-extension forms.

## Mathlib compatibility

* Prepare the language specification for an eventual Mathlib port.
* Specify how Lean's universe levels are represented.
* Specify how Lean's universe constraints are preserved.
* PRP: Specify the interpretation of Lean's `Prop`.
  * Account for impredicative quantification.
  * Account for definitional proof irrelevance.
  * Account for elimination restrictions.
* QUT: Specify quotients or an equivalent encoding.
  * Define quotient formation.
  * Define quotient introduction.
  * Define quotient elimination.
  * Define quotient soundness.
  * Define quotient computation.
* AXM: Choose the classical foundation.
  * State the status of choice.
  * State the status of propositional extensionality.
  * State which principles are axioms.
  * State which principles are derived.
  * Distinguish noncomputable definitions from executable computations.
* PRT: Define the proof-preservation contract.
  * Choose between translating source, translating elaborated proofs, and reproving translated statements.
  * Preserve theorem statements.
  * Preserve axiom dependencies.
  * Fix dependency versions.
  * Specify checking of translated proofs.

## References

* [Lean type system](https://lean-lang.org/doc/reference/latest/The-Type-System/)
* [Lean functions](https://lean-lang.org/doc/reference/latest/The-Type-System/Functions/)
* [Lean universes](https://lean-lang.org/doc/reference/latest/The-Type-System/Universes/)
* [Lean inductive types](https://lean-lang.org/doc/reference/latest/The-Type-System/Inductive-Types/)
* [Lean propositions](https://lean-lang.org/doc/reference/latest/The-Type-System/Propositions/)
* [Lean quotients](https://lean-lang.org/doc/reference/latest/The-Type-System/Quotients/)
* [Lean axioms](https://lean-lang.org/doc/reference/latest/Axioms/)
