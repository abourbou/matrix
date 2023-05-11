
use std::ops::{Add, Sub, Mul, Div};
use std::fmt::Display;

pub trait Scalar: Add + Sub + Mul + Div + PartialEq + Copy + Display
{
    fn zero() -> Self;
    fn one() -> Self;
	fn norm(self) -> f32;
}

impl Scalar for f32 {
    fn zero() -> f32 {
        0.0
    }
    fn one() -> f32 {
        1.0
    }
	fn norm(self) -> f32 {
		self.abs()
	}
}

