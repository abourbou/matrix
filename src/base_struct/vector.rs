
#[derive(Clone, PartialEq)]
pub struct Vector {
	pub vec: Vec<f32>,
	pub size: usize,
}

impl<const N: usize> From<[f32; N]> for Vector {
	fn from(d: [f32; N]) -> Self {
		if N == 0 {
			panic!("try to creat an empty vector");
		}
		Self { vec : Vec::<f32>::from(d), size: d.len()}
	}
}

impl Vector {
	pub fn new(number: f32, size: usize) -> Self {
		if size == 0 {
			panic!("null size for vector creation");
		}
		Vector {vec: vec![number; size], size: size}
	}

	pub fn size(&self) -> usize {
		self.size
	}

	pub fn print(&self) {
		for i in 0..self.size {
			println!("[{}]", self.vec[i]);
		}
		println!();
	}
}