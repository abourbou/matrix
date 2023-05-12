
extern crate matrixlib;

use matrixlib::base_struct::matrix::Matrix;
use matrixlib::base_struct::vector::Vector;
use num_complex::Complex;

fn main() {
	let mut mat = Matrix::<f32,3,2>::new(3.);

	println!("{}", mat);
	mat.print();

	let mat_comp = Matrix::<Complex<f32>,3,2>::new(Complex::<f32>::new(3., -1.));
	println!("{}", mat_comp);
	mat_comp.print();

	let mat3 = 2.0 * mat;
	mat += mat3;

	println!("{}", mat);

	let vec = Vector::<f32,2>::new(2.5);
	println!("{}", vec);
}