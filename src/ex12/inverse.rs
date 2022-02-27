
use crate::base_struct::Matrix;

impl Matrix {
	pub fn inverse(self) -> Result<Matrix, &'static str> {
		if !self.is_square() {
			panic!("none square matrix doesn't have an inverse");
		}
		let [m, n] = self.shape();
		//check determinant
		if self.determinant() == 0. {
			return Err("Singular matrix, no inverse");
		}

		//creation identity matrix
		let mut result_matrix = Matrix::new(0., m, m);
		result_matrix.mat = result_matrix.mat.iter().enumerate()
		.map(|(it, x)| {
			if it / m == it % m {
				return x + 1.;
			}
			return *x;
		}).collect();

		//row reduce the matrix
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
				result_matrix.swap_line(pivot, r);
				buffer_matrix.swap_line(pivot, r);
			}
			if (&buffer_matrix).mat[r * n + j] != 1. {
				result_matrix.mult_line(pivot, 1. / buffer_matrix.mat[r * n + j]);
				buffer_matrix.mult_line(pivot, 1. / buffer_matrix.mat[r * n + j]);
			}

			//reduce other lines
			for i in 0..m {
				if i == pivot {
					continue;
				}
				if buffer_matrix.mat[i * n + j] != 0. {
					result_matrix.add_line(i, r, -1. * buffer_matrix.mat[i * n + j] / buffer_matrix.mat[r * n + j]);
					buffer_matrix.add_line(i, r, -1. * buffer_matrix.mat[i * n + j] / buffer_matrix.mat[r * n + j]);
				}
			}
			r += 1;
		}
		Ok(result_matrix)
	}
}