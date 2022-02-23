
use super::linear_combination;
use crate::base_struct::Vector;

pub fn main01()
{
	//*test of linear combination
	//! linear_combination(&[], &[2.]);
	//! linear_combination(&[Vector::from([1.,1.,1.])], &[2.]).print();
	// !linear_combination(&[Vector::from([1.,1.,1.]), Vector::from([2.,2.])], &[0., 1.]).print();
	linear_combination(&[Vector::from([1.,1.,1.])], &[2.]).print();
	linear_combination(&[Vector::from([1.,1.,1.]), Vector::from([4., -5., 8.5])], &[3., 2.]).print();
}