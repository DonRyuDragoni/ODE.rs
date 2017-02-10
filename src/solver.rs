use traits::*;

pub enum Method {
    RK2,
    RK4,
}

/**
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
    where T: Number<T>,
          F: Fn(&T, &Vec<T>) -> Vec<T> {
    method: Method,
    weights: Vec<T>,
    weight_sum: T,
    step: T,
    initial_conditions: Vec<T>,
    function: F,
}

impl<T, F> Solver<T, F>
    where T: Number<T>,
          F: Fn(&T, &Vec<T>) -> Vec<T> {

    /**
    Start building a new Solver.
    */
    pub fn new(initial_conditions: &Vec<T>, function: F) -> Solver<T, F> {
        Solver {
            method:             Method::RK4,
            weights:            vec![T::from(1), T::from(2),
                                     T::from(2), T::from(1)],
            weight_sum:         T::from(6),
            step:               T::from(10e-3),
            initial_conditions: initial_conditions.clone(),
            function:           function,
        }
    }

    /**
    Select the desired algorithm to solve the givem problem, also changing the
    weights based on the selected case:

    - `Method::RK2`: `vec![T::from(1), T::from(1)],`
    - `Method::RK4`: `vec![T::from(1), T::from(2), T::from(2), T::from(1)],`

    Defaults to `Method::RK4`.

    To manually alter these values, please check `Self::change_weight()`.
    */
    pub fn method(&mut self, new_method: Method) -> &mut Solver<T, F> {
        match new_method {
            Method::RK2 => self.weights = vec![T::from(1), T::from(1)],
            Method::RK4 => self.weights = vec![T::from(1), T::from(2),
                                               T::from(2), T::from(1)],
        }

        self.weight_sum = self.weights.iter()
            .fold(T::from(0), |sum, i| sum + i.clone());
        self.method = new_method;
        self
    }

    /**
    Modify the default weighting applyed to each intermidiate point.
    */
    pub fn change_weight(&mut self, new_weights: Vec<T>) -> &mut Solver<T, F> {
        self.weights = new_weights.clone();

        let mut sum: T = T::from(0);
        for i in new_weights.iter() {
            sum = sum + i.clone();
        }

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
