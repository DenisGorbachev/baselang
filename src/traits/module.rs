use crate::VarRc;

pub trait Module {
    type RefsTuple<'a>
    where
        Self: 'a;

    fn vars(&self) -> Vec<VarRc>;

    fn refs_tuple(&self) -> Self::RefsTuple<'_>;

    fn print(&self) -> Vec<String> {
        self.vars().into_iter().map(|var| var.print()).collect()
    }
}

#[macro_export]
macro_rules! vars {
    ($($name:ident),+) => {
        fn vars(&self) -> Vec<VarRc> {
            vec![$(self.$name.clone()),+]
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

            $crate::vars!($($var),+);

            $crate::refs_tuple!($($var),+);
        }
    };
}
