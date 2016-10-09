use std::ops::Add;

/**
Basic requirements for a type to use the solver.
*/
pub trait Number<T> : Add<Output = T> + Clone + From<u8> + From<f32> {}

/**
Anything that implements the required traits already implements Number.
*/
impl<T> Number<T> for T
    where T: Add<Output = T> + Clone + From<u8> + From<f32> {}

/**
Function to be solved by this library.
*/
pub trait Function<T> : Fn(&T, &Vec<T>) -> Vec<T>
    where T: Number<T> {}
