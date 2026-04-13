# Aist

Aist is a Rust package that provides an ergonomic API for `TyCtxt`.

## Concepts

### Wrapper

A struct that wraps a type from an internal compiler crate.

- Must derive `Deref`
- Must have a lifetime parameter `'c`
- Must have a wrapped value field (examples: `AdtDef`, `FieldDef`)
  - Must have a `#[deref]` attribute
- Must have a `tcx: TyCtxt<'c>` field
  - Must be the last field
- Should have methods that provide access to related values
  - Examples:
    - `Adt` wrapper should have a `field` method
    - `Adt` wrapper should have a `fields` method
  - Requirements:
    - Methods should accept `impl IntoSymbol` instead of `DefId`
