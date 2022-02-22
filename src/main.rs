
mod base_struct;
use crate::base_struct::{Vector, Matrix};

//#[cfg(feature = "ex00")]
mod ex00;

fn base()
{
	//matrix creation
	let vec = Vector::new(0., 3);
	vec.print();
	let vec = Vector::from([0., 1., 2.]);
	vec.print();

	let mat = Matrix::new(0., 5, 5);
	mat.print();
	let mat = Matrix::from([[0.,0.,0.], [1.,1.,1.]]);
	mat.print();
	println!("matrix shape : {:?}", mat.shape());
	println!("matrix is square : {}", mat.is_square());
}

fn ex00()
{
	//vector base operations
	println!("\n\nBase Vector Operations test : ");
	let mut vec1 = Vector::from([1.,2.,3.]);
	let vec2 = Vector::from([1.,1.,1.]);
	vec1.add(&vec2);
	vec1.print();
	vec1.sub(&vec2);
	vec1.print();
	vec1.scl(2.);
	vec1.print();

	//matrix base operations
	println!("\n\nBase Vector Operations test : ");
	let mut mat1 = Matrix::from([[0.,1.],[2.,3.]]);
	let mat2 = Matrix::new(1., 2, 2);
	mat1.add(&mat2);
	mat1.print();
	mat1.sub(&mat2);
	mat1.print();
	mat1.scl(2.);
	mat1.print();
}

fn main() {

	base();
	ex00();
}


//TODO add conditionnal compilation
// https://github.com/bwasty/learn-opengl-rs/blob/master/src/main.rs