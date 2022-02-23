
use crate::base_struct::{Vector, Matrix};

pub fn main00()
{
	//*vector base operations
	println!("\n\nVector Operations test : ");
	let mut vec1 = Vector::from([1.,2.,3.]);
	let vec2 = Vector::from([1.,1.,1.]);
	vec1.add(&vec2);
	vec1.print();
	vec1.sub(&vec2);
	vec1.print();
	vec1.scl(2.);
	vec1.print();

	//*matrix base operations
	println!("\n\nMatrix Operations test : ");
	let mut mat1 = Matrix::from([[0.,1.],[2.,3.]]);
	let mat2 = Matrix::new(1., 2, 2);
	mat1.add(&mat2);
	mat1.print();
	mat1.sub(&mat2);
	mat1.print();
	mat1.scl(2.);
	mat1.print();
}