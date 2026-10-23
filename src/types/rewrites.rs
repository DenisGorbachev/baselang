use crate::{Rewrite, Sym};
use std::collections::HashMap;
use std::ptr::NonNull;

pub type Rewrites = HashMap<NonNull<Sym>, Vec<Rewrite>>;
