
mod super::matrix;
use matrix::Matrix;


pub struct Vector {
	vec: Vec<f32>,
	size: usize,
}

impl Vector {
	pub fn new(number: f32, size: usize) -> Self {
		let vec = vec![number; size];
		if size == 0 {
			panic!("null size for vector creation");
		}
		Vector {vec: vec, size: size}
	}

	pub fn from(array: &[f32]) -> Self {
		let vec : Vec<f32> = array.to_vec();
		let size : usize = vec.len();

		if size == 0 {
			panic!("null size for vector creation");
		}
		Vector {vec: vec, size: size}
	}



	// pub fn to_matrix(&self) -> Matrix<f32> {

	// }

	pub fn size(&self) -> usize {
		self.size
	}

	pub fn print(&self) {
		for i in 0..self.size {
			println!("[{}]", self.vec[i]);
		}
	}
}