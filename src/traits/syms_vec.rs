use crate::{Rewrite, SymRc};

pub trait SymsVec {
    fn syms_vec(&self) -> Vec<&SymRc>;

    fn rewrites_vec(&self) -> Vec<(&SymRc, &Rewrite)> {
        Vec::new()
    }
}

#[macro_export]
macro_rules! impl_syms_vec {
    ($name:ident, $($field:ident),+ $(,)?; $scope:ident) => {
        impl $crate::SymsVec for $name {
            fn syms_vec(&self) -> Vec<&$crate::SymRc> {
                vec![$(&self.$field),+]
            }

            fn rewrites_vec(&self) -> Vec<(&$crate::SymRc, &$crate::Rewrite)> {
                self.syms_vec().into_iter().flat_map(|sym| self.$scope.rewrites_of(sym).iter().map(move |rule| (sym, rule))).collect()
            }
        }
    };
    ($name:ident, $($field:ident),+ $(,)?) => {
        impl $crate::SymsVec for $name {
            fn syms_vec(&self) -> Vec<&$crate::SymRc> {
                vec![$(&self.$field),+]
            }

        }
    };
}

#[macro_export]
macro_rules! impl_syms_vec_aggregate {
    ($name:ident, $($field:ident),+ $(,)?) => {
        impl $crate::SymsVec for $name {
            fn syms_vec(&self) -> Vec<&$crate::SymRc> {
                $crate::concat_with_extend(vec![
                    $($crate::SymsVec::syms_vec(&self.$field)),+
                ])
            }

            fn rewrites_vec(&self) -> Vec<(&$crate::SymRc, &$crate::Rewrite)> {
                $crate::concat_with_extend(vec![$($crate::SymsVec::rewrites_vec(&self.$field)),+])
            }
        }
    };
}
