use std::fmt::Debug;
use num_traits::Num;

/**
 * Basic requirements for a type to be usable by solver.
 */
pub trait Number: Num + PartialOrd + Debug + Clone + Copy {}

/**
 * Anything that implements the required traits already implements Number.
 */
impl<T> Number for T
where
    T: Num + PartialOrd + Debug + Clone + Copy,
{
}

/**
 * Function to be solved by this library.
 */
pub trait Function<T>: Fn(&T, &Vec<T>) -> Vec<T>
where
    T: Number,
{
}

impl<T, F> Function<T> for F
where
    T: Number,
    F: Fn(&T, &Vec<T>) -> Vec<T>,
{
}
