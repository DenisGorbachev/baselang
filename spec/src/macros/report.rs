#[macro_export]
macro_rules! report {
    ($vis:vis struct $name:ident { $($(#[cfg($cfg:meta)])? $field_vis:vis $field:ident: $typ:ty),+ $(,)? }) => {
        #[derive(Debug)]
        $vis struct $name {
            $($(#[cfg($cfg)])? $field_vis $field: $typ),+
        }

        impl $crate::Report for $name {
            fn passed(&self) -> bool {
                let passed = true;
                $(
                    $(#[cfg($cfg)])?
                    let passed = passed && $crate::Report::passed(&self.$field);
                )+
                passed
            }

            fn outcomes(&self) -> impl Iterator<Item = $crate::CheckOutcome<'_>> {
                let outcomes = std::iter::empty();
                $(
                    $(#[cfg($cfg)])?
                    let outcomes = outcomes.chain($crate::Report::outcomes(&self.$field).map(|mut outcome| { outcome.prepend_parent(stringify!($field)); outcome }));
                )+
                outcomes
            }
        }
    };
}
