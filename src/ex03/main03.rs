
use crate::base_struct::Vector;

pub fn main03() {
	let u = Vector::from([0., 0.]);
	let v = Vector::from([0., 0.]);
	println!("{}", u.dot(&v));
	let u = Vector::from([1., 0.]);
	let v = Vector::from([1., 0.]);
	println!("{}", u.dot(&v));
	let u = Vector::from([4., 2.]);
	let v = Vector::from([2., 1.]);
	println!("{}", u.dot(&v));

}