/*!
Baselang modules are implemented as Rust structs with [`VarRc`] fields.

We use the term "mod" to refer to either Baselang modules or Rust modules. It should always be obvious from the context whether we refer to Baselang modules or Rust modules.

The name of the root export field, after camel-case transformation, must be equal to the name of the module. For example, the `Nat` module must have a field `nat`, which must contain a [`VarRc`] with the `Nat` type.
This makes it more convenient for users of the module who can pattern-match on the module struct to extract the variables with correct names (no need to rename them).
*/

mod list;

pub use list::*;

mod bool;

pub use bool::*;

mod nat;

pub use nat::*;

mod prelude;

pub use prelude::*;
