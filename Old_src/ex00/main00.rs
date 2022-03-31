
use crate::base_struct::{Vector, Matrix};
use std::time::Instant;

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

	let start = Instant::now();
	const  BIG_SIZE: usize = 10000 * 1;
	let mut vec3 = Vector::from([1.5; BIG_SIZE]);
	let vec4 = Vector::from([0.75; BIG_SIZE]);
	for _ in 0..1000 {
		vec3.add(&vec4);
		vec3.sub(&vec4);
		vec3.scl(2.);
	}
	let duration = start.elapsed();
	println!("time to run 1000 time a vector of {} elements : {}s", BIG_SIZE, duration.as_millis());

	//test overload operator
	println!("\n\nTest overload functions");
	let mut vec5 = vec1 + vec2;
	vec5.print();
	vec5 = vec5 - Vector::from([0., 1., 2.]);
	vec5.print();
	vec5 = vec5 * 2.;
	vec5.print();
	vec5 = 0.5 * vec5;
	vec5.print();
	
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
	let start = Instant::now();
	let mut mat3 = Matrix::from([[1.5; BIG_SIZE], [1.5; BIG_SIZE], [1.5; BIG_SIZE]]);
	let mat4 = Matrix::from([[0.75; BIG_SIZE], [0.75; BIG_SIZE], [0.75; BIG_SIZE]]);
	for _ in 0..1000 {
		mat3.add(&mat4);
		mat3.sub(&mat4);
		mat3.scl(2.);
	}
	let duration = start.elapsed();
	println!("time to run 1000 time a matrix of {}  by 3 elements : {}s", BIG_SIZE, duration.as_millis());

	//test overload operator
	println!("\n\nTest overload functions");
	let mut mat5 = mat1 + mat2;
	mat5.print();
	mat5 = mat5 - Matrix::from([[0.,1.],[2.,3.]]);
	mat5.print();
	mat5 = mat5 * 2.;
	mat5.print();
	mat5 = 0.5 * mat5;
	mat5.print();
}