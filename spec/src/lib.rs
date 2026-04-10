#![feature(rustc_private)]
#![deny(clippy::arithmetic_side_effects)]

extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

mod facts;

pub use facts::*;

mod helpers;

pub use helpers::*;
