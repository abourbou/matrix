
use crate::base_struct::Vector;

impl Vector {
	pub fn add(&mut self, v: &Vector) {
		if v.size() != self.size() {
			panic!("can't add different size vectors");
		}
		self.vec = self.vec.iter().zip(v.vec.iter()).map(|(&s, &v)| s + v).collect();
	}
	pub fn sub(&mut self, v: &Vector) {
		if v.size() != self.size() {
			panic!("can't sub different size vectors");
		}
		self.vec = self.vec.iter().zip(v.vec.iter()).map(|(&s, &v)| s - v).collect();
	}
	pub fn scl(&mut self, k: f32) {
		self.vec = self.vec.iter().map(|&s| k * s).collect();
	}
}

