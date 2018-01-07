# ODE.rs

[![Published Version](https://img.shields.io/crates/v/ode.svg)](https://crates.io/crates/ode)
[![Documentation](https://docs.rs/ode/badge.svg)](https://docs.rs/ode/)

[![Build Status](https://travis-ci.org/DonRyuDragoni/ODE.rs.svg?branch=master)](https://travis-ci.org/DonRyuDragoni/ODE.rs)
[![Coverage Status](https://coveralls.io/repos/github/DonRyuDragoni/ODE.rs/badge.svg?branch=master)](https://coveralls.io/github/DonRyuDragoni/ODE.rs?branch=master)

Provide generic solvers for ODEs using different methods. Also, allow the user
to choose which datatype they desire to use for that problem, be it `f32`, `u8`
or some non-standard bignum type.

By no means this is yet ready for any serious use right now. The code needs to
be cleaned and there is a lot yet to be implemented.

## Example

```rust
use ode::{Method, Solver};

// The interval in time you wish to calculate.
let time_interval: [f32; 2] = [0., 100.];

// The value of y(t = 0).
let initial_condition: Vec<f32> = vec![0.];

// You can configure the solver in a single line:
let _ = Solver::new(&time_interval, &initial_condition)
    .method(Method::RK4);

// Or with multiple statements:
let mut solver = Solver::new(&time_interval, &initial_condition);
solver.method(Method::RK4);

// To run the solver, simply pass the closure of your choice. In this case, I'm
// integrating
//
//     y(t) = 2 * t
let (times, pos) = solver.solve(|t: &f32, _: &Vec<f32>| {
    vec![2.*t]
});
```

## Current Goals

### For the next minor version (0.2.0)

- [ ] more and better documentation;
- [x] re-implement the 4th order Runge-Kutta method;
- [ ] have at least a few tests and examples;
- [ ] stabilize the API.

### For the next major version (1.0.0)

- [ ] figure out a generic way to write all Runge-Kutta variants that is both
  easy to maintain and clean to read;
- [ ] implement all planned Runge-Kutta variants:
  - [ ] 2;
  - [ ] 3;
  - [x] 4;
  - [ ] 5.

## [Release Notes](./Release%20Notes.md)

## [Contributing](./CONTRIBUTING.md)

If you whant to help this project, check the [Contributing](./CONTRIBUTING.md)
file for detailed information. (Spoiler: you're welcome to contribute in any way
you feel you can help!)
