
use crate::base_struct::Matrix;

impl Matrix {

	pub fn determinant(self) -> f32 {
		if !self.is_square() {
			panic!("cannot calculate determinant of a none square matrix");
		}
		let [m, n] = self.shape();
		if m == 1 {
			self.mat[0]
		}
		else if m == 2 {
			self.mat[0] * self.mat[3] - self.mat[1] * self.mat[2]
		}
		else {
			let mut r : usize = 0;
			let mut det = 1.;
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
					det *= -1.;
				}
				//reduce other lines
				for i in r + 1..m {
					if buffer_matrix.mat[i * n + j] != 0. {
						buffer_matrix.add_line(i, r, -1. * buffer_matrix.mat[i * n + j] / buffer_matrix.mat[r * n + j]);
					}
				}
				r += 1;
			}
			return buffer_matrix.mat.iter().enumerate().filter(|(it, _x)| it % m == it / m).map(|(_it, x)| x).product::<f32>() * det;
		}
	}
}