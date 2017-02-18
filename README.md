# ODE.rs

[![Build Status](https://travis-ci.org/DonRyuDragoni/ODE.rs.svg?branch=master)](https://travis-ci.org/DonRyuDragoni/ODE.rs)
[![Published Version](https://img.shields.io/crates/v/ode.svg)](https://crates.io/crates/ode)
[![Documentation](https://docs.rs/ode/badge.svg)](https://docs.rs/ode/)

Provide generic solvers for ODEs using different methods. Also, allow the user
to choose which datatype they desire to use for that problem, be it `f32`, `u8`
or some non-standard bignum type..

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

## Release Notes

- 0.?.? [**WIP**]

- 0.1.2: break change

    Change the Number trait to
    use [num_traits](https://crates.io/crates/num-traits) crate, so that
    accepted types are more consistent with what is already present in the
    community.

    To keep consistency with function names, `solver::Solver::change_weight()`
    is now called `solver::Solver::weights()`.

    Also, a bit more of documentation.

    RK4 is back online, and the only usable method so far.

- 0.1.1 [**unusable**]: break change

    Closing issue #1, this drops the MATLAB-ish style in favor of a more Rustic
    way of doing things. This also starts the process of building version 0.2.0,
    attempting to stabilize the API.

    This is not a usable version, as `solver::Solver::run()` will always return
    empty vectors.

- 0.1.0: first draft

    Started the project as an attempt to bring MATLAB-ish ODE solvers to
    Rust. The API is completelly unstable and may change at any time.

    Only the RK4 method is available, with a very basic usage.
