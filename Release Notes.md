# Release Notes

## 0.?.? [**WIP**]

- Change the example in the `README` file to reflect the changes made in the
  previous version.

## 0.1.2

- Change the `Number` trait to use `Num` in the [num_traits][] crate, so that
  accepted types are more consistent with what is already present in the
  community.

- A bit more of documentation (still needs improvement, though; like, a metric
  ton of it, maybe more).

- RK4 is back online, and the only usable method so far, dropping the `unusable`
  shame flag of the previous version.

### break changes

- To keep consistency with function names, `solver::Solver::change_weight()` is
  now called `solver::Solver::weights()`.

- `solver::Solver::run()` was changed to `solver::Solver::solve()`, and expects
  the function to be solved instead of nothing.

[num_traits]: https://crates.io/crates/num-traits

## 0.1.1 [unusable]

- Closing issue #1, this drops the MATLAB-ish style in favor of a more Rustic
  way of doing things. This also starts the process of building version 0.2.0,
  attempting to stabilize the API.

### break changes

- This is not a usable version, as `solver::Solver::run()` will always return
  empty vectors (before, `rk4::solver()` would actually run the simulation).

## 0.1.0

- Started the project as an attempt to bring MATLAB-ish ODE solvers to Rust. The
  API is completelly unstable and may change at any time.

- Only the RK4 method is available, with a very basic usage.
