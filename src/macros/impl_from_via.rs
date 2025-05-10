#[macro_export]
macro_rules! impl_from_via {
    ($source:ty, $intermediary:ty, $target:ident) => {
        impl From<$source> for $target {
            fn from(value: $source) -> Self {
                Into::<$intermediary>::into(value).into()
            }
        }
    };
    (impl From<$source:ty> via From<$intermediary:ty> for $target:ident) => {
        $crate::impl_from_via!($source, $intermediary, $target);
    };
}
