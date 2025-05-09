use crate::VarRc;

pub trait Module {
    type RefsTuple<'a>
    where
        Self: 'a;

    fn vars_refs(&self) -> Vec<&VarRc>;

    fn refs_tuple(&self) -> Self::RefsTuple<'_>;
}

#[macro_export]
macro_rules! vars_refs {
    ($($name:ident),+) => {
        fn vars_refs(&self) -> Vec<&VarRc> {
            vec![$(&self.$name),+]
        }
    };
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
            pub $($var: $crate::VarRc),+
        }

        impl $crate::Module for $name {
            type RefsTuple<'a> = $crate::refs_tuple_type!($($var),+);
            // type RefsTuple<'a> = $crate::RefsTuple4<'a>;

            $crate::vars_refs!($($var),+);

            $crate::refs_tuple!($($var),+);
        }
    };
}
