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

let time_interval: [f32; 2] = [0., 100.];
let ini_cond: Vec<f32> = vec![0.];

// simple config
Solver::new(&time_interval, &ini_cond)
    .method(Method::RK4)
    .solve(|t: &f32, _: &Vec<f32>| vec![2.*t]);

// complex config
let mut s = Solver::new(&time_interval, &ini_cond);
s.method(Method::RK4);

// run the solver
let (times, pos) = s.solve(|t: &f32, _: &Vec<f32>| vec![2.*t]);
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

## [Release Notes](./Release_Notes.md)

## [Contributing](./CONTRIBUTING.md)

If you whant to help this project, check the [Contributing](./CONTRIBUTING.md)
file for detailed information. (Spoiler: you're welcome to contribute in any way
you feel you can help!)
