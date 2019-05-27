//! The core crate for [`kurobako`](https://github.com/sile/kurobako).
#[macro_use]
extern crate log;
#[macro_use]
extern crate trackable;

pub use error::{Error, ErrorKind};

pub mod epi;
pub mod filter;
pub mod json;
pub mod num;
pub mod parameter;
pub mod problem;
pub mod solver;

mod error;

/// This crate specific `Result` type.
pub type Result<T> = std::result::Result<T, Error>;
