
use super::lerp;
use crate::base_struct::{Vector, Matrix};

pub fn main02()
{
	println!("{}", lerp(0., 1., 0.));
	println!("{}", lerp(0., 1., 1.));
	println!("{}", lerp(0., 1., 0.5));
	println!("{}", lerp(21., 42., 0.3));
	println!("{}", lerp(Vector::from([2., 1.]), Vector::from([4., 2.]), 0.3));
	println!("{}", lerp(Matrix::from([[2., 1.], [3., 4.]]), Matrix::from([[20.,
		10.], [30., 40.]]), 0.5));
		
}