use num_traits::Num;

/**
Basic requirements for a type to be usable by solver.
*/
pub trait Number : Num + Clone {}

/**
Anything that implements the required traits already implements Number.
*/
impl<T> Number for T
    where T: Num + Clone {}

/**
Function to be solved by this library.
*/
pub trait Function<T> : Fn(&T, &Vec<T>) -> Vec<T>
    where T: Number {}
