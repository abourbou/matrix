
use crate::base_struct::Matrix;

impl Matrix {
	pub fn add(&mut self, m: &Matrix) {
		if m.shape() != self.shape() {
			panic!("can't add different size matrices");
		}
		self.mat = self.mat.iter().zip(m.mat.iter()).map(|(&s, &m)| s + m).collect();
	}
	pub fn sub(&mut self, m: &Matrix) {
		if m.shape() != self.shape() {
			panic!("can't sub different size matrices");
		}
		self.mat = self.mat.iter().zip(m.mat.iter()).map(|(&s, &m)| s - m).collect();
	}
	pub fn scl(&mut self, k: f32) {
		self.mat = self.mat.iter().map(|&s| k * s).collect();
	}
}