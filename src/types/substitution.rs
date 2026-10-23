use crate::{Exp, Sym};
use std::collections::HashMap;
use std::ptr::NonNull;

pub type Substitution = HashMap<NonNull<Sym>, Exp>;
