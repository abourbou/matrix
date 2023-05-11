
use crate::base_struct::{scalar::Scalar, vector::Vector};
use std::ops::{Add, Mul};

impl <T : Scalar, const M : usize> Vector<T,M>
	where T : Add<T,Output=T> + Mul<T,Output=T>{
	pub fn dot(&self, v: &Self) -> T {
		let mut dot = T::zero();
		for i in 0..M {
			dot = dot + self.arr[i] * v.arr[i];
		}
		dot
	}
}

#[cfg(test)]
	#[test]
	fn test00() {
		let u = Vector::from([0., 0.]);
		let v = Vector::from([0., 0.]);
		assert_eq!(u.dot(&v), 0.);
	}
	#[test]
	fn test01() {
		let u = Vector::from([1., 0.]);
		let v = Vector::from([1., 0.]);
		assert_eq!(u.dot(&v), 1.);
	}
	#[test]
	fn test02() {
		let u = Vector::from([4., 2.]);
		let v = Vector::from([2., 1.]);
		assert_eq!(u.dot(&v), 10.);
	}
	#[test]
	fn test03() {
		let u = Vector::from([-3., 6., 4.25]);
		let v = Vector::from([2., 1.65, 1.]);
		assert_eq!(u.dot(&v), 8.15);
	}
