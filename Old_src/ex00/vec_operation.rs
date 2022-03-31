
use crate::base_struct::Vector;
use std::ops;

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

//impl overload operator
impl ops::Add<Vector> for Vector {
	type Output = Vector;

	fn add(self, _rhs: Vector) -> Vector {
		if _rhs.size() != self.size() {
			panic!("can't add different size matrices");
		}
		Vector {
			vec: self.vec.iter().zip(_rhs.vec.iter()).map(|(&s, &r)| s + r).collect(),
			size: self.size,
		}
	}
}

impl ops::Sub<Vector> for Vector {
	type Output = Vector;

	fn sub(self, _rhs: Vector) -> Vector {
		if _rhs.size() != self.size() {
			panic!("can't add different size matrices");
		}
		Vector {
			vec: self.vec.iter().zip(_rhs.vec.iter()).map(|(&s, &r)| s - r).collect(),
			size: self.size,
		}
	}
}

impl ops::Mul<f32> for Vector {
	type Output = Vector;

	fn mul(self, _rhs: f32) -> Vector {
		Vector {
			vec: self.vec.iter().map(|&s| s * _rhs).collect(),
			size: self.size,
		}
	}
}

impl ops::Mul<Vector> for f32 {
	type Output = Vector;

	fn mul(self, _rhs: Vector) -> Vector {
		Vector {
			vec: _rhs.vec.iter().map(|&s| s * self).collect(),
			size: _rhs.size(),
		}
	}
}


