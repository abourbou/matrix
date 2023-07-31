
#[macro_use]
extern crate assert_float_eq;

pub mod base;
pub use base::{matrix::Matrix, vector::Vector};
pub mod graphic_operations;
pub mod linear_operations;
pub mod vectorial_space;


// !Later
// TODO Add matrix operation from scop
// TODO Add accessor for the lib to the array and put it not pub ?
// TODO Try Deref/DerefMut for Vector instead of reimplementing everything
// TODO Use MaybeUninit<T> to init array
// TODO Use iterator MUCH more