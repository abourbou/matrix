
use crate::base_struct::{scalar::Scalar, vector::Vector};
use std::ops::{Add, Mul};

impl <T : Scalar, const M : usize> Vector<T,M>
	where T : Add<T,Output=T> + Mul<T,Output=T>{
	pub fn norm_1(&self) -> f32 {
		let mut norm = T::zero().norm();
		for i in 0..M {
			norm = norm + self.arr[i].norm();
		}
		norm
	}

	pub fn norm(&self) -> f32 {
		let mut norm = T::zero().norm();
		for i in 0..M {
			norm = norm + self.arr[i].norm() * self.arr[i].norm();
		}
		norm.sqrt()
	}

	pub fn norm_inf(&self) -> f32 {
		let mut norm = T::zero().norm();
		for i in 0..M {
			let buffer = self.arr[i].norm();
			if buffer > norm {
				norm = buffer;
			}
		}
		norm
	}
}

#[cfg(test)]
	#[test]
	fn test00() {
		let u = Vector::from([0., 0., 0.]);
		assert_eq!(u.norm_1(), 0.);
		assert_eq!(u.norm(), 0.);
		assert_eq!(u.norm_inf(), 0.);
	}
	#[test]
	fn test01() {
		let u = Vector::from([1., 2., 3.]);
		assert_eq!(u.norm_1(), 6.);
		assert_eq!(u.norm(), f32::sqrt(14.));
		assert_eq!(u.norm_inf(), 3.);
	}
	#[test]
	fn test02() {
		let u = Vector::from([-1., -2.]);
		assert_eq!(u.norm_1(), 3.0);
		assert_eq!(u.norm(), f32::sqrt(5.0));
		assert_eq!(u.norm_inf(), 2.);
	}