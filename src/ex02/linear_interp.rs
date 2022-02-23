

use crate::base_struct::{Vector, Matrix};

mov Vector {
	pub fn lerp(u: Vector, v: Vector, t: f32) -> Vector {
		if u.size() != v.size() {
			panic!("can not interpolate vectors of different sizes");
		}
		let vec : Vec<f32> = Vec::new();
		let opposite = 1. - t;
		for i in 0..u.size() {
			vec.push(u.vec[i] * opposite + v.vec[i] * t);
		}
		Vector {vec: vec, size: u.size}
	}
}