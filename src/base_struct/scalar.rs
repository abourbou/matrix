
use num_traits::NumAssign;
use std::fmt::Display;


pub trait Scalar : Copy + NumAssign + Display {
	fn norm(self) -> f32;
}

impl Scalar for f32 {
	fn norm(self) -> f32 {
		self.abs()
	}
}