use crate::SymRc;

pub trait SymsVec {
    fn syms_vec(&self) -> Vec<&SymRc>;
}

#[macro_export]
macro_rules! impl_syms_vec {
    ($name:ident, $($field:ident),+) => {
        impl $crate::SymsVec for $name {
            fn syms_vec(&self) -> Vec<&$crate::SymRc> {
                vec![$(&self.$field),+]
            }
        }
    };
}

#[macro_export]
macro_rules! impl_syms_vec_aggregate {
    ($name:ident, $($field:ident),+) => {
        impl $crate::SymsVec for $name {
            fn syms_vec(&self) -> Vec<&$crate::SymRc> {
                $crate::concat_with_extend(vec![
                    $($crate::SymsVec::syms_vec(&self.$field)),+
                ])
            }
        }
    };
}
