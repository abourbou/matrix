
use crate::base_struct::Vector;

impl Vector {
	pub fn dot(&self, v: &Vector) -> f32 {
		self.vec.iter().zip(v.vec.iter()).map(|(&s, &v)| s * v).sum()
	}
}
