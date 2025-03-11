pub trait Of<T> {
    type Output;

    fn of(&self, rhs: T) -> Self::Output;
}
