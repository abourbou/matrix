

use crate::base_struct::Vector;

impl Vector {
	pub fn add(&mut self, v: &Vector) {
		if v.size() != self.size() {
			panic!("can't add different size vectors");
		}
		for i in 0..self.size() {
			self.vec[i] += v.vec[i];
		}
	}
	pub fn sub(&mut self, v: &Vector) {
		if v.size() != self.size() {
			panic!("can't sub different size vectors");
		}
		for i in 0..self.size() {
			self.vec[i] -= v.vec[i];
		}
	}
	pub fn scl(&mut self, k: f32) {
		for i in 0..self.size() {
			self.vec[i] *= k;
		}
	}
}

