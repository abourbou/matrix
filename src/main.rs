
mod vector;
use vector::Vector;

mod matrix;
use matrix::Matrix;

fn main() {
	let vec = Vector::new(0., 3);
	vec.print();
	let vec = Vector::from([0., 1., 2.]);
	vec.print();

	let mat = Matrix::new(0., 5, 5);
	mat.print();
}
