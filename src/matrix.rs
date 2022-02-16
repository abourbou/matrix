
pub struct Matrix {
	mat: Vec<Vec<f32>>,
	size_rows: usize,
	size_col: usize,
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
		Matrix {mat: mat, size_rows: row, size_col: col}
	}

	pub fn shape(&self) -> [usize; 2] {
		[self.size_rows, self.size_col]
	}

	pub fn is_square(&self) -> bool {
		self.size_rows == self.size_col
	}

	pub fn print(&self) {
		for i in 0..self.size_rows {
			for j in 0..self.size_col {
				if j == 0 {
					print!("[");
				}
				print!(" {}", self.mat[i][j]);
				if j != self.size_col - 1 {
					print!(",");
				}
				else {
					println!(" ]");
				}
			}
		}
	}
}