
use crate::base_struct::Matrix;
use std::ops;

//impl methods

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

//impl overload operator
impl ops::Add<Matrix> for Matrix {
	type Output = Matrix;

	fn add(self, _rhs: Matrix) -> Matrix {
		if _rhs.shape() != self.shape() {
			panic!("can't add different size matrices");
		}
		Matrix {
			mat: self.mat.iter().zip(_rhs.mat.iter()).map(|(&s, &r)| s + r).collect(),
			number_rows: self.number_rows,
			number_cols: self.number_cols
		}
	}
}

impl ops::Sub<Matrix> for Matrix {
	type Output = Matrix;

	fn sub(self, _rhs: Matrix) -> Matrix {
		if _rhs.shape() != self.shape() {
			panic!("can't add different size matrices");
		}
		Matrix {
			mat: self.mat.iter().zip(_rhs.mat.iter()).map(|(&s, &r)| s - r).collect(),
			number_rows: self.number_rows,
			number_cols: self.number_cols
		}
	}
}

impl ops::Mul<f32> for Matrix {
	type Output = Matrix;

	fn mul(self, _rhs: f32) -> Matrix {
		Matrix {
			mat: self.mat.iter().map(|&s| s * _rhs).collect(),
			number_rows: self.number_rows,
			number_cols: self.number_cols
		}
	}
}

impl ops::Mul<Matrix> for f32 {
	type Output = Matrix;

	fn mul(self, _rhs: Matrix) -> Matrix {
		Matrix {
			mat: _rhs.mat.iter().map(|&s| s * self).collect(),
			number_rows: _rhs.number_rows,
			number_cols: _rhs.number_cols
		}
	}
}
