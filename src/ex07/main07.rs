
use crate::base_struct::{Vector, Matrix};

pub fn main07() {

	println!("multiplication with vector");
	let u = Matrix::from([
		[1., 0.],
		[0., 1.],
		]);
	let v = Vector::from([4., 2.]);
	println!("{}", u.mul_vec(&v));
	// [4.]
	// [2.]

	let u = Matrix::from([
	[2., 0.],
	[0., 2.],
	]);
	let v = Vector::from([4., 2.]);
	println!("{}", u.mul_vec(&v));
	// [8.]
	// [4.]

	let u = Matrix::from([
	[2., -2.],
	[-2., 2.],
	]);
	let v = Vector::from([4., 2.]);
	println!("{}", u.mul_vec(&v));
	// [4.]
	// [-4.]

	println!("\nmultiplication with mat");
	let u = Matrix::from([
	[1., 0.],
	[0., 1.],
	]);
	let v = Matrix::from([
	[1., 0.],
	[0., 1.],
		]);
	println!("{}", u.mul_mat(&v));
	// [1., 0.]
	// [0., 1.]

	let u = Matrix::from([
	[1., 0.],
	[0., 1.],
	]);
	let v = Matrix::from([
	[2., 1.],
	[4., 2.],
	]);
	println!("{}", u.mul_mat(&v));
	// [2., 1.]
	// [4., 2.]

	let u = Matrix::from([
	[3., -5.],
	[6., 8.],
	]);
	let v = Matrix::from([
	[2., 1.],
	[4., 2.],
	]);
	println!("{}", u.mul_mat(&v));
	// [-14., -7.]
	// [44., 22.]

}