use crate::VarRc;

pub trait VarsVec {
    fn vars_vec(&self) -> Vec<&VarRc>;
}

#[macro_export]
macro_rules! impl_vars_vec {
    ($name:ident, $($field:ident),+) => {
        impl $crate::VarsVec for $name {
            fn vars_vec(&self) -> Vec<&$crate::VarRc> {
                vec![$(&self.$field),+]
            }
        }
    };
}

#[macro_export]
macro_rules! impl_vars_vec_aggregate {
    ($name:ident, $($field:ident),+) => {
        impl $crate::VarsVec for $name {
            fn vars_vec(&self) -> Vec<&$crate::VarRc> {
                $crate::concat_with_extend(vec![
                    $($crate::VarsVec::vars_vec(&self.$field)),+
                ])
            }
        }
    };
}
