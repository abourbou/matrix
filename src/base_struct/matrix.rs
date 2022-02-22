
pub struct Matrix {
	pub mat: Vec<Vec<f32>>,
	pub size_rows: usize,
	pub size_cols: usize,
}

impl<const M: usize, const N: usize> From<[[f32; M]; N]> for Matrix {
	fn from(arr_of_arr: [[f32; M]; N]) -> Self {
		let mut mat : Vec<Vec<f32>> = Vec::new();
		if M == 0 || N == 0 {
			panic!("null size for matrix creation");
		}
		for &arr in &arr_of_arr {
			let vec = Vec::<f32>::from(arr);
			mat.push(vec);
		}
	    Self {mat : mat, size_rows: N, size_cols: M}
	}
}


impl Matrix {
	pub fn new(number: f32, row: usize, col: usize) -> Self {
		let mut mat : Vec<Vec<f32>> = Vec::new();
		if row == 0 || col == 0 {
			panic!("null size for matrix creation");
		}
		for _i in 0..row
		{
			let vec = vec![number; col];
			mat.push(vec);
		}
		Matrix {mat: mat, size_rows: row, size_cols: col}
	}

	pub fn shape(&self) -> [usize; 2] {
		[self.size_rows, self.size_cols]
	}

	pub fn is_square(&self) -> bool {
		self.size_rows == self.size_cols
	}

	pub fn print(&self) {
		for i in 0..self.size_rows {
			for j in 0..self.size_cols {
				if j == 0 {
					print!("[");
				}
				print!(" {}", self.mat[i][j]);
				if j != self.size_cols - 1 {
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