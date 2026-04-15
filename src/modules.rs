/*!
# Modules

Baselang modules are implemented as Rust structs with [`VarRc`](crate::VarRc) fields.

TODO: Modules are isomorphic to functions that expose both the inputs and the outputs (inputs are other modules, vars, names, counts; outputs are vars)

Module passes the following validations:

* Module is a Rust struct
* Every field of a module passes the following validations:
    * Fields is `pub`
    * Field is a [`VarRc`](crate::VarRc)
* Module has a `pub fn new` which returns `Self`
    * Some modules don't implement `Default` because they accept other modules as arguments to their `pub fn new`
    * If a module doesn't accept any arguments to `pub fn new`, it should implement `Default`
* The name of the root export field, after camel-case transformation, is equal to the name of the module.
    * Example: `Nat` module has a field `nat` with [`VarRc`] type. The Rust field `Nat::nat` holds the Baselang `Nat` type.
    * Notes
        * This makes it more convenient for users of the module who can pattern-match on the module struct to extract the variables with correct names (no need to rename them).

The term "mod" may refer to either a Baselang module or a Rust module. It should always be obvious from the context whether it refers to a Baselang module or Rust module.
*/

mod lists;

pub use lists::*;

mod bools;

pub use bools::*;

mod nats;

pub use nats::*;

mod vectors;

pub use vectors::*;

mod prelude;

pub use prelude::*;

mod test_prelude;

pub use test_prelude::*;

mod sum;

pub use sum::*;

mod measure;

pub use measure::*;

mod units;

pub use units::*;

mod wats;

pub use wats::*;

mod ints;

pub use ints::*;

mod rats;

pub use rats::*;
