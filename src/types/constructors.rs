use crate::{Sym, SymRc};
use std::collections::HashMap;
use std::ptr::NonNull;

pub type Constructors = HashMap<NonNull<Sym>, Vec<SymRc>>;
