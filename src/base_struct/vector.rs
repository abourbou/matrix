
//use std::fmt;

// #[derive(Clone, PartialEq, Debug)]
// pub struct Vector<T>
// {
// 	pub vec: [T, N],
// 	pub size: usize,
// }

// impl<const N: usize> From<[f32; N]> for Vector {
// 	fn from(d: [f32; N]) -> Self {
// 		if N == 0 {
// 			panic!("try to create an empty vector");
// 		}
// 		Self { vec : Vec::<f32>::from(d), size: d.len()}
// 	}
// }

// impl Vector {
// 	pub fn new(number: f32, size: usize) -> Self {
// 		if size == 0 {
// 			panic!("null size for vector creation");
// 		}
// 		Vector {vec: vec![number; size], size}
// 	}

// 	pub fn size(&self) -> usize {
// 		self.size
// 	}

// 	pub fn print(&self) {
// 		for i in 0..self.size {
// 			println!("[{}]", self.vec[i]);
// 		}
// 		println!();
// 	}
// }

// impl fmt::Display for Vector {
// 	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
// 			let mut vec_string = String::new();
// 			for i in 0..self.size {
// 				vec_string.push('[');
// 				vec_string.push_str(&self.vec[i].to_string());
// 				vec_string.push_str("]\n");
// 			}
// 			write!(f, "{}", vec_string)
// 	}
// }