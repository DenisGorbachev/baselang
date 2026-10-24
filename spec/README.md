# Baselang executable specification

Run `mise --cd ~/workspace/baselang run spec:check` from any directory.

The executable reports every observed predicate and returns a nonzero exit status for a violation, panic, missing declaration, failed operation, or unverified universal obligation. It does not modify the implementation. Rebuild it after changing Baselang so that structural inspection and executed code describe the same revision.

`src/declarations.rs` specifies declaration shapes, generic defaults, aliases, derives, signatures, and hygienic macro expansions. These are syntax contracts over the library declarations loaded by rust-analyzer, including macro expansions. They intentionally prescribe canonical declarations; they do not establish semantic equivalence between arbitrarily different Rust programs.

The other modules define executable equations and finite witnesses for application, registration, rewrites, normalization, parsing, theory validation, rendering, and macro evaluation. `src/scope_invariants.rs` defines scope predicates over bindings, rule types, and constructor indexes. Reports retain underlying errors and identify failed equations.

Finite witnesses are not universal proofs. The report explicitly leaves universal invariant preservation, the validity relation of every `Apply` implementation, and the missing `DuoVec` invariant unverified. Missing behavior must produce a failure rather than an empty successful report. Existing Baselang mismatches remain visible; the specification is not weakened to fit them.

Reusable report aggregation, probes, source inspection, declaration comparison, and Cargo checks live in `aist-core` in the standalone aist repository. The deprecated compiler wrapper is replaced by a normal executable; there is no automatic fix operation.
