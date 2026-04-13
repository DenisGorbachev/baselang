#![feature(rustc_private)]
#![deny(clippy::arithmetic_side_effects)]

extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

mod outcome;
pub use outcome::*;

mod with_tcx;
pub use with_tcx::*;

mod get_mutators;
pub use get_mutators::*;

mod into_symbol;
pub use into_symbol::*;

mod ctx;
pub use ctx::*;

mod adt;
pub use adt::*;

mod field;
pub use field::*;
