
use crate::base_struct::Vector;

impl Vector {
	pub fn norm_1(&self) -> f32 {
		self.vec.iter().map(|v| v.abs()).sum()
	}

	pub fn norm(&self) -> f32 {
		self.vec.iter().map(|v| v * v).sum::<f32>().powf(0.5)
	}

	pub fn norm_inf(&self) -> f32 {
		self.vec.iter().map(|v| v.abs()).reduce(|accum, item| {
			if accum >= item {accum} else {item}
		}).expect("norm_inf cannot be perform with NaN values")
	}
}