#![doc(html_root_url = "https://docs.rs/ode/0.1.2")]

extern crate num_traits;

mod traits;
mod rk4;
mod solver;

// re-exports

pub use traits::*;
pub use solver::*;
