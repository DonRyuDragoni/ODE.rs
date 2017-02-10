use std::ops::Add;

/**
Basic requirements for a type to be usable by solver.

`From<u8>` and `From<f32>` are needed to build the weights and the step for the
Solver. For more details, see the source at `solver::Solver::new()`.
*/
pub trait Number<T> : Add<Output = T>
    + Clone
    + From<u8> + From<f32> {}

/**
Anything that implements the required traits already implements Number.
*/
impl<T> Number<T> for T
    where T: Add<Output = T>
    + Clone
    + From<u8> + From<f32> {}

/**
Function to be solved by this library.
*/
pub trait Function<T> : Fn(&T, &Vec<T>) -> Vec<T>
    where T: Number<T> {}
