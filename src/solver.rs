use traits::*;

pub enum Method {
    RK2,
    RK4,
}

/**

# Default values

When creating a new `Solver` with `Solver::new()`, this is the default
configuration:

- [method](struct.Solver.html#method.method): `Method::RK4`

- [weights](struct.Solver.html#method.weights) for the intermediate points:

    - `Method::RK2`: `vec![1, 1],`
    - `Method::RK4`: `vec![1, 2, 2, 1],`

(Note that each internal value of the vector will be converted to `T` via
[T::from_str_radix()](../num_traits/trait.Num.html#tymethod.from_str_radix).)

# Example

```
use ode::{Method, Solver};

let ini_cond: Vec<f32> = vec![1., 2.];

// simple config
Solver::new(&ini_cond, |t: &f32, _: &Vec<f32>| vec![2.*t])
    .method(Method::RK4)
    .run();

// complex config
let mut s = Solver::new(&ini_cond, |t: &f32, _: &Vec<f32>| vec![2.*t] );
s.method(Method::RK4);

// run the solver
let (times, pos) = s.run();
```
*/
pub struct Solver<T, F>
    where T: Number,
          F: Fn(&T, &Vec<T>) -> Vec<T> {
    method: Method,
    weights: Vec<T>,
    weight_sum: T,
    step: T,
    initial_conditions: Vec<T>,
    function: F,
}

impl<T, F> Solver<T, F>
    where T: Number,
          F: Fn(&T, &Vec<T>) -> Vec<T> {

    /*
    Re-calculate the sum of the registered weights.
    */
    fn sum_weights(weights: &Vec<T>) -> T {
        weights
            .iter()
            .fold(T::zero(), |sum, i| sum + i.clone())
    }

    /*
    Return the default weights for the intermidiate points of a given method.

    Return values are:

    - `Method::RK2`: `vec![1, 1],`
    - `Method::RK4`: `vec![1, 2, 2, 1],`

    (Note that each internal value of the vector will be converted to `T` via
    `T::from_str_radix()`.)
    */
    fn get_default_weights_for(method: &Method) -> Vec<T> {
        let weights = match method {
            &Method::RK2 => vec!["1", "1"],
            &Method::RK4 => vec!["1", "2", "2", "1"],
        };

        weights
            .iter()
            .map(|el| T::from_str_radix(el, 10).ok().unwrap())
            .collect()
    }

    /**
    Start building a new Solver.

    Default values are:

    - method: `Method::RK4`,
    - intermidiate point weights: `vec![1, 2, 2, 1]`
    - step size: `10e-3`

    (Note that these values will be converted to `T` via `T::from_str_radix()`.)
    */
    pub fn new(initial_conditions: &Vec<T>, function: F) -> Solver<T, F> {
        let weights = Self::get_default_weights_for(&Method::RK4);
        let sum_of_weights = Self::sum_weights(&weights);
        let default_step = T::from_str_radix("10e-3", 10).ok().unwrap();

        Solver {
            method:             Method::RK4,
            weights:            weights,
            weight_sum:         sum_of_weights,
            step:               default_step,
            initial_conditions: initial_conditions.clone(),
            function:           function,
        }
    }

    /**
    Select the desired algorithm to solve the givem problem, also updating the
    weights based on default values. To manually alter these values, please
    check `Self::change_weight()`.
    */
    pub fn method(&mut self, new_method: Method) -> &mut Solver<T, F> {
        self.weights = Self::get_default_weights_for(&new_method);
        self.method = new_method;
        self.weight_sum = Self::sum_weights(&self.weights);

        self
    }

    /**
    Modify the default weighting applyed to each intermidiate point.
    */
    pub fn weights(&mut self, new_weights: Vec<T>) -> &mut Solver<T, F> {
        self.weights = new_weights.clone();
        self.weight_sum = Self::sum_weights(&self.weights);

        self
    }

    /**
    Validates the data passed to the solver.

    This is tipically called by `run()` before attempting to solve the passed
    function, but was made public so the data can be verified and corrected
    without a panic.

    Data checks are:

    - `Self::weights.len()` should match the selected method:

        - `Method::RK2` requires 2 weights;
        - `Method::RK4` requires 4 weights;
    */
    pub fn validate(&self) -> bool {
        self.weights.len() == match self.method {
            Method::RK2 => 2,
            Method::RK4 => 4,
        }
    }

    /**
    Check and run the solver, returning the results of the calculation.

    Notice that, before the solver actually runs, it validates its data by
    calling `Self::validate()` within an `assert!`. See that function for more
    details.
    */
    pub fn run(&self) -> (Vec<T>, Vec<Vec<T>>) {
        assert!(self.validate());

        (Vec::new(), Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let _ = Solver::new(&vec![0., 0.],
                            |t: &f32, _: &Vec<f32>| -> Vec<f32> {
                                vec![2.*t]
                            });
    }
}
