
use crate::base_struct::Matrix;

impl Matrix {
	pub fn add(&mut self, m: &Matrix) {
		if m.shape() != self.shape() {
			panic!("can't add different size matrices");
		}
		for i in 0..self.size_rows {
			for j in 0..self.size_cols {
				self.mat[i][j] += m.mat[i][j];
			}
		}
	}
	pub fn sub(&mut self, m: &Matrix) {
		if m.shape() != self.shape() {
			panic!("can't sub different size matrices");
		}
		for i in 0..self.size_rows {
			for j in 0..self.size_cols {
				self.mat[i][j] -= m.mat[i][j];
			}
		}
	}
	pub fn scl(&mut self, k: f32) {
		for i in 0..self.size_rows {
			for j in 0..self.size_cols {
				self.mat[i][j] *= k;
			}
		}
	}
}