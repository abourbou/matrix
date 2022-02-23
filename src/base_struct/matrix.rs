#[derive(Clone, PartialEq)]
pub struct Matrix {
	pub mat: Vec<f32>,
	pub number_rows: usize,
	pub number_cols: usize,
}

impl<const M: usize, const N: usize> From<[[f32; N]; M]> for Matrix {
	fn from(arr_of_arr: [[f32; N]; M]) -> Self {
		if M == 0 || N == 0 {
			panic!("try to create an empty matrix");
		}
		let mat : Vec<f32> = arr_of_arr.into_iter().flatten().collect();
	    Self {mat : mat, number_rows: M, number_cols: N}
	}
}

impl Matrix {
	pub fn new(number: f32, row: usize, col: usize) -> Self {
		if row == 0 || col == 0 {
			panic!("null size for matrix creation");
		}
		Matrix {mat: vec![number; row * col], number_rows: row, number_cols: col}
	}

	pub fn shape(&self) -> [usize; 2] {
		[self.number_rows, self.number_cols]
	}

	pub fn is_square(&self) -> bool {
		self.number_rows == self.number_cols
	}

	pub fn print(&self) {
		for i in 0..self.number_rows {
			for j in 0..self.number_cols {
				if j == 0 {
					print!("[");
				}
				print!(" {}", self.mat[i * self.number_cols + j]);
				if j != self.number_cols - 1 {
					print!(",");
				}
				else {
					println!(" ]");
				}
			}
		}
		println!();
	}
}