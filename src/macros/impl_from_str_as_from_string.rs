#[macro_export]
macro_rules! impl_from_str_as_from_string {
    ($id:ident) => {
        impl From<&str> for $id {
            fn from(value: &str) -> Self {
                Self::from(String::from(value))
            }
        }
    };
}
