use crate::VarsVec;

pub trait Module: VarsVec {
    type RefsTuple<'a>
    where
        Self: 'a;

    fn refs_tuple(&self) -> Self::RefsTuple<'_>;
}

#[macro_export]
macro_rules! refs_tuple {
    ($($name:ident),+) => {
        fn refs_tuple(&self) -> Self::RefsTuple<'_> {
            ($(&self.$name),+)
        }
    };
}

#[macro_export]
macro_rules! module {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $($var:ident),+$(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Eq, PartialEq, Hash, Clone, Debug)]
        $vis struct $name {
            $(pub $var: $crate::VarRc),+
        }

        $crate::impl_vars_vec!($name, $($var),+);

        impl $crate::Module for $name {
            type RefsTuple<'a> = $crate::refs_tuple_type!($($var),+);
            // type RefsTuple<'a> = $crate::RefsTuple4<'a>;

            $crate::refs_tuple!($($var),+);
        }
    };
}
