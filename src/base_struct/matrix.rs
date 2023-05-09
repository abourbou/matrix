
//use std::fmt;

use super::scalar::Scalar; 

#[derive(Debug)]
pub struct Matrix<T : Scalar, const M: usize, const N: usize>
{
	pub arr: [[T; N]; M],
	pub number_rows: usize,
	pub number_cols: usize,
}


impl<T : Scalar, const M: usize, const N: usize> From<[[T; N]; M]> for Matrix<T, M, N> {
	fn from(arr: [[T; N]; M]) -> Self {
		if M == 0 || N == 0 {
			panic!("try to create an empty matrix");
		}
	    Self {arr, number_rows: M, number_cols: N}
	}
}

// impl Matrix {
// 	pub fn new(number: f32, row: usize, col: usize) -> Self {
// 		if row == 0 || col == 0 {
// 			panic!("null size for matrix creation");
// 		}
// 		Matrix {mat: vec![number; row * col], number_rows: row, number_cols: col}
// 	}

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
	use super::Matrix;

	#[test]
	fn basic_test00() {
		let arr = [[0.,0.], [0.,0.]];
		let mat = Matrix::from(arr);
		assert_eq!(mat.arr, arr);
		assert_eq!(mat.number_rows, 2);
		assert_eq!(mat.number_cols, 2);
	}
	#[test]
	fn basic_test01() {
		let arr = [[0.,1.]];
		let mat = Matrix::from(arr);
		assert_eq!(mat.arr, arr);
		assert_eq!(mat.number_rows, 1);
		assert_eq!(mat.number_cols, 2);
	}
	#[test]
	fn basic_test02() {
		let arr = [[0.], [1.]];
		let mat = Matrix::from(arr);
		assert_eq!(mat.arr, arr);
		assert_eq!(mat.number_rows, 2);
		assert_eq!(mat.number_cols, 1);
	}
	#[test]
	#[should_panic]
	fn basic_test03() {
		let arr : [[f32; 0];1] = [[]];
		let _mat = Matrix::from(arr);
	}
	#[test]
	#[should_panic]
	fn basic_test04() {
		let arr : [[f32; 1];0] = [];
		let _mat = Matrix::from(arr);
	}


}