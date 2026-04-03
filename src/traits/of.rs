use crate::{Exp, InvalidApplicationError};

/// This trait makes it easier to construct [`Exp`](crate::Exp) from [`VarRc`](crate::VarRc) or [`ExpRc`](crate::ExpRc).
///
/// This trait accepts `&self` instead of `self` because there's never a need to take ownership of the receiver.
/// But is it true? What if Exp is applied on Exp - should we create an app from two owned exps or not?
///
/// It's better to have a separate trait than to implement the `of` method directly on `Var` or `Exp` because we need the `of` method on `VarRc` and `ExpRc` (which are considered foreign types, because they are specifications of [`Rc`](std::rc::Rc)).
///
/// This trait is called `Of` instead of `App` to distinguish it from the `App` variant
pub trait Of<T> {
    /// Applies `self` to `rhs`
    fn of(&self, rhs: T) -> Result<Exp, InvalidApplicationError>;

    /// Applies `self` to `rhs`, treating `rhs` as the `index` arg
    fn of_at(&self, rhs: T, index: usize) -> Result<Exp, InvalidApplicationError>;

    /// Applies `self` to `rhs`, trying to find the right index for `rhs`
    fn of_smart(&self, rhs: T) -> Option<Exp>;
}

pub fn assert_impl_of<Fun, Arg>(fun: &Fun, arg: Arg)
where
    Fun: Of<Arg>,
    Arg: Clone,
{
    assert_eq_of_at_zero(fun, arg.clone());
}

pub fn assert_eq_of_at_zero<Fun, Arg>(fun: &Fun, arg: Arg)
where
    Fun: Of<Arg>,
    Arg: Clone,
{
    let of_arg = arg.clone();
    let of_at_arg = arg;
    let of_result = fun.of(of_arg);
    let of_at_result = fun.of_at(of_at_arg, 0);
    assert_eq!(of_result, of_at_result)
}
