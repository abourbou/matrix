
use crate::base_struct::{scalar::Scalar, vector::Vector};
use std::ops::{Add, Mul};

impl<T : Scalar, const M: usize> Vector<T, M>
	where T: Add<T,Output=T> + Mul<T,Output=T>{
	pub fn linear_combination(u: &[Self], coeffs: &[T]) -> Self{
		if u.is_empty() {
			panic!("empty list of vector");
		}
		else if u.len() != coeffs.len() {
			panic!("list coeff need to be the same size that the list of vector");
		}

		let mut vec_lincb = Self::default();
		for i in  0..M {
			let mut buffer_value : T = T::zero();
			for j in 0..u.len() {
				buffer_value = coeffs[j] * u[j].arr[i] + buffer_value;
			}
		vec_lincb.arr[i] = buffer_value;
	}
	vec_lincb
	}
}

#[cfg(test)]
	#[test]
	fn linear_comb_test00() {
		let vec = Vector::from([1.0, 2.0, 3.0]);
		let vec2 = Vector::<f32,3>::new(2.0);
		let u = [vec, vec2];
		let coeffs = [2.0, 1.0];
		assert_eq!(Vector::<f32,3>::linear_combination(&u, &coeffs), Vector::from([4.0, 6.0, 8.0]));
	}
	#[test]
	fn linear_comb_test01() {
		let vec = Vector::from([1.0, 2.0]);
		let vec2 = Vector::<f32,2>::new(2.0);
		let u = [vec, vec2];
		let coeffs = [0.0, 3.0];
		assert_eq!(Vector::<f32,2>::linear_combination(&u, &coeffs), Vector::from([6.0, 6.0]));
	}
	#[test]
	#[should_panic]
	fn linear_comb_test02() {
		let vec = Vector::from([1.0, 2.0]);
		let vec2 = Vector::<f32,2>::new(2.0);
		let u = [vec, vec2];
		let coeffs = [1.0];
		assert_eq!(Vector::<f32,2>::linear_combination(&u, &coeffs), Vector::from([6.0, 6.0]));
	}
	#[test]
	#[should_panic]
	fn linear_comb_test03() {
		let u = [];
		let coeffs = [1.0];
		assert_eq!(Vector::<f32,2>::linear_combination(&u, &coeffs), Vector::from([6.0, 6.0]));
	}
