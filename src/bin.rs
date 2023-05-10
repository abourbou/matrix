
extern crate matrixlib;

use matrixlib::base_struct::matrix::Matrix;
use matrixlib::base_struct::vector::Vector;

fn main() {
	let mat = Matrix::<f32,3,2>::new(3.);
	mat.print();
	println!("{}", mat);
	let vec = Vector::<f32,2>::new(2.5);
	vec.print();
	println!("{}", vec);
}