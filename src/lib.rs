
#[macro_use]
extern crate assert_float_eq;

pub mod base_class;
pub mod graphic_operations;
pub mod linear_operations;
pub mod vectorial_space;


// !Later
// TODO reorganize the lib to be a "real" matrix operation and not a list of ex
// TODO rename fills to be explicit
// TODO Add matrix operation from scop
// TODO Add accessor for the lib to the array and put it not pub ?
// TODO Try Deref?DerefMut for Vector instead of reimplementing everything
// TODO Use MaybeUninit<T> to init array
// TODO Use iterator MUCH more