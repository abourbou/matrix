
use super::linear_combination;
use crate::base_struct::Vector;
use std::time::Instant;

pub fn main01()
{
	//*test of linear combination
	//! linear_combination(&[], &[2.]);
	//! linear_combination(&[Vector::from([1.,1.,1.])], &[2.]).print();
	// !linear_combination(&[Vector::from([1.,1.,1.]), Vector::from([2.,2.])], &[0., 1.]).print();
	linear_combination(&[Vector::from([1.,1.,1.])], &[2.]).print();
	linear_combination(&[Vector::from([1.,1.,1.]), Vector::from([4., -5., 8.5])], &[3., 2.]).print();

	let start = Instant::now();
	let mut big_vec_vector : Vec<Vector> = Vec::new();
	const BIG_SIZE : usize = 1000000 * 1;
	for _ in 0..BIG_SIZE {
		big_vec_vector.push(Vector::from([1., 2., 3., 4., 5., 6., 7., 8., 9.]).clone());
	}
	linear_combination(big_vec_vector.as_slice(), &[1.;BIG_SIZE]).print();
	let duration = start.elapsed();
	println!("time to run a array of {} elements : {}ms", BIG_SIZE, duration.as_millis());
}