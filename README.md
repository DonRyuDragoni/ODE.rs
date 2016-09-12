# ODE.rs

[![Build Status](https://travis-ci.org/DonRyuDragoni/ODE.rs.svg?branch=master)](https://travis-ci.org/DonRyuDragoni/ODE.rs)

Provide MATLAB-ish solvers for ODEs using different methods. Also, allow the
user to choose which datatype (s)he desires to use for that problem.

By no means this is yet ready for any serious use right now. The code needs to
be cleaned and there is a lot yet to be implemented.

## TODOs

- [ ] implement solver for rk2, rk3 and rk5
- [ ] custom weights for each implementation (e.g.: rk4 defaults to 1-2-2-1, but
  the user may whant 1-3-3-1 for some reason)
- [ ] provide default values for said weights and for the step
