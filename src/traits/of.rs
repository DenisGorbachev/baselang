use crate::{Exp, InvalidApplicationError};

/// This trait makes it easier to construct [`Exp`](crate::Exp) from [`VarRc`](crate::VarRc) or [`ExpRc`](crate::ExpRc).
///
/// This trait accepts `&self` instead of `self` because there's never a need to take ownership of the receiver.
/// But is it true? What if Exp is applied on Exp - should we create an app from two owned exps or not?
///
/// It's better to have a separate trait than to implement the `of` method directly on `Var` or `Exp` because we need the `of` method on `VarRc` and `ExpRc` (which are considered foreign types, because they are specifications of [`Rc`](std::rc::Rc)).
pub trait Of<T> {
    fn of(&self, rhs: T) -> Result<Exp, InvalidApplicationError>;
}
