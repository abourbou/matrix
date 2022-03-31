
use crate::base_struct::Matrix;

impl Matrix {
	pub fn trace(&self) -> f32 {
		if !self.is_square() {
			panic!("none square matrix cannot have a trace");
		}
		let m = self.number_rows;
		self.mat.iter().enumerate().filter(|(it, _x)| it / m == it % m).map(|(_it, x)| x).sum()
	}
}