
//use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub struct Matrix<T, const M: usize, const N: usize>
	(pub [[T; N]; M]);

impl <T, const M: usize, const N: usize> From<[[T; N]; M]> for Matrix<T, M, N> {
	fn from(item: [[T; N]; M]) -> Self {
		Matrix (item)
	}
}
// }
// 	pub fn shape(&self) -> [usize; 2] {
// 		[self.number_rows, self.number_cols]
// 	}

// 	pub fn is_square(&self) -> bool {
// 		self.number_rows == self.number_cols
// 	}

// 	pub fn print(&self) {
// 		for i in 0..self.number_rows {
// 			for j in 0..self.number_cols {
// 				if j == 0 {
// 					print!("[");
// 				}
// 				print!(" {}", self.mat[i * self.number_cols + j]);
// 				if j != self.number_cols - 1 {
// 					print!(",");
// 				}
// 				else {
// 					println!(" ]");
// 				}
// 			}
// 		}
// 		println!();
// 	}
// }

// impl fmt::Display for Matrix {
// 	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
// 		let mut mat_string = String::new();
// 		for i in 0..self.number_rows {
// 			for j in 0..self.number_cols {
// 				if j == 0 {
// 					mat_string.push('[');
// 				}
// 				//mat_string.push_str(" ");
// 				mat_string.push_str(&self.mat[i * self.number_cols + j].to_string());
// 				if j != self.number_cols - 1 {
// 					mat_string.push_str(", ");
// 				}
// 				else {
// 					mat_string.push(']');
// 				}
// 			}
// 			mat_string.push('\n');
// 		}
// 		write!(f, "{}", mat_string)
// 	}
// }

#[cfg(test)]
mod tests {
	use crate::base_struct::Matrix;
	#[test]
	fn test_valid_matrix() {
		let u = Matrix([[1]]);
		assert_eq!(u.0, 0);
	}
	let w = Matrix([[1,2],[3,4]]);
	let v = Matrix([[1,2]]);
}