
extern crate matrixlib;

use matrixlib::base_struct::matrix::Matrix;
use matrixlib::base_struct::vector::Vector;

fn main() {
	let mut mat = Matrix::<f32,3,2>::new(3.);
	let mut vec = Vector::<f32,2>::new(2.5);

	let mat3 = 2.0 * mat;
	// println!("{}", mat3);

	mat += mat3;
	println!("{}", mat);
	mat -= mat3;
	println!("{}", mat);
	mat *= 4.0;
	println!("{}", mat);

	let vec2 = Vector::<f32,2>::new(2.5);
	vec += vec2;
	println!("{}", vec);

}