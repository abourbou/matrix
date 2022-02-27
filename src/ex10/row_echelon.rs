
use crate::base_struct::Matrix;

impl Matrix {
	//swap line : swap two lines of a matrix
	// L1 <-> L2
	pub fn swap_line(&mut self, nb_line1 : usize, nb_line2: usize) {
		let [m,n] = self.shape();
		if nb_line1 >= m || nb_line2 >= m {
			panic!("line's index of the matrix overflow");
		}
		if nb_line1 == nb_line2 {
			return;
		}
		let mut buffer : f32;
		for j in 0..n {
			buffer = self.mat[nb_line2 * n + j];
			self.mat[nb_line2 * n + j] = self.mat[nb_line1 * n + j];
			self.mat[nb_line1 * n + j] = buffer;
		}
	}
	//add_line : add a line multiplied by a coeff to an other line
	//L_changed <- L_changed + coeff * L_added
	pub fn add_line(&mut self, line_changed : usize, line_added : usize, coeff: f32) {
		let [m,n] = self.shape();
		if line_changed >= m || line_added >= m {
			panic!("line's index of the matrix overflow");
		}
		for j in 0..n {
			self.mat[line_changed * n + j] = self.mat[line_changed * n + j] + coeff * self.mat[line_added * n +j];
		}
	}
	//mult_line: multiply a line of a matrix by a coeff
	// L_changed <- coeff * L_changed
	pub fn mult_line(&mut self, line_changed : usize, coeff: f32) {
		let [m,n] = self.shape();
		if line_changed >= m {
			panic!("line's index of the matrix overflow");
		}
		for j in 0..n {
			self.mat[line_changed * n + j] = self.mat[line_changed * n + j] * coeff;
		}
	}

	pub fn row_echelon(self) -> Matrix {
		let [m,n] = self.shape();
		let mut r : usize = 0;
		let mut buffer_matrix = self.clone();
		for j in 0..n {
			if r == m {
				break;
			}
			//find pivot
			let mut some_pivot : Option<usize> = None;
			for i in r..m {
				if (&buffer_matrix).mat[i * n + j] != 0. {
					some_pivot = Some(i);
					break;
				}
			}
			if some_pivot == None {
				continue;
			}

			//operations on pivot
			let pivot = some_pivot.unwrap();
			if pivot != r {
				buffer_matrix.swap_line(pivot, r);
			}
			if (&buffer_matrix).mat[r * n + j] != 1. {
				buffer_matrix.mult_line(pivot, 1. / buffer_matrix.mat[r * n + j]);
			}

			//reduce other lines
			for i in 0..m {
				if i == pivot {
					continue;
				}
				if buffer_matrix.mat[i * n + j] != 0. {
					buffer_matrix.add_line(i, r, -1. * buffer_matrix.mat[i * n + j] / buffer_matrix.mat[r * n + j]);
				}
			}
			r += 1;
		}
		buffer_matrix
	}
}