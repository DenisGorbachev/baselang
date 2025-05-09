/*!
Baselang is a language for describing how things work.

This crate provides an embedded domain-specific language (eDSL) for modeling and type-checking concepts in a way that's both precise and expressive.

## Core Concepts

- **Types**: Represented by [`Typ`], which can be top-level types [`Top`], expression types [`One`],
  or function types [`Fun`].
- **Expressions**: Represented by [`Exp`], which can be variables [`Sol`] or applications [`App`].
- **Variables**: Represented by [`Var`], which have names and types.
- **Modules**: Collections of related variables, like [`Bool`], [`List`], and [`Nat`].
- **Type Checking**: Enforced at construction time to ensure type-safety across the language.

## Features

- Static type checking at compile time and runtime
- Composable modules that model foundational concepts
- Macros for concise expression construction
*/

mod types;

pub use types::*;

mod modules;

pub use modules::*;

mod errors;

pub use errors::*;

mod traits;

pub use traits::*;

mod render;

pub use render::*;

#[cfg(test)]
mod tests;

#[cfg(test)]
pub use tests::*;

mod utils;

pub use utils::*;
