# Baselang executable specification

Run `mise --cd ~/workspace/baselang run spec:check` from any directory. The spec has its own Cargo workspace, lockfile, and Rust toolchain. It does not link to Baselang.

`src/declarations.rs` inspects the separately supplied Baselang project through rust-analyzer. It requires declaration shapes, generic defaults, aliases, derives, method signatures, hygienic macro expansions, and specific test function names. The `Typ::Fun` field representation is intentionally unconstrained.

Behavioral checks live in Baselang's `src/tests/contracts` and run with Baselang's tests. They cover application, registration, rewrites, rollback, normalization, parsing, theory validation, rendering, and macro evaluation. The smart application contract is explicitly ignored until `Exp::of_smart` is implemented. Requiring a test declaration does not establish that the test passes or is enabled.

`DuoVec<T>` stores a two-element prefix and a vector tail. Its invariant is `self.iter().take(2).count() == 2`. The prefix and tail remain private. `first()` and `last()` return references without a panic branch. Conversion from a short vector returns its original elements in the error.

Semantic invariant reviews are behind the implicit `aist-codex` feature and are disabled by default. Review the implementation in the aist repository before enabling them, as recorded in Cargo.toml. They check invariant declarations and preservation across constructors, conversions, deserializers, public mutation, and mutable aliases, plus the validity contract of `Apply`. An LLM judgment is not a machine-checked proof. Default structural checks do not claim to verify these obligations.

When explicitly enabled, the runner requires a model, cache directory, maximum cache age, and review timeout. It reports failed and inconclusive reviews as failures. `--help` lists the available settings. Cache identity includes captured source, dependency features, compilation context, and review settings; see [aist-codex](../../aist/packages/aist-codex/README.md) for invalidation boundaries and external-input requirements.

Reports retain underlying errors and return a nonzero exit status for violations or failed operations. Reusable reporting, source inspection, and declaration comparison live in `aist-core`. Codex execution, snapshots, and caching live in `aist-codex`.
