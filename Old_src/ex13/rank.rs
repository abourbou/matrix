
use crate::base_struct::Matrix;

impl Matrix {
	pub fn rank(mut self) -> usize{
		let [m,n] = self.shape();
		let mut r : usize = 0;
		for j in 0..n {
			if r == m {
				break;
			}
			//find pivot
			let mut some_pivot : Option<usize> = None;
			for i in r..m {
				if self.mat[i * n + j] != 0. {
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
				self.swap_line(pivot, r);
			}
			//reduce other lines
			for i in r + 1..m {
				if self.mat[i * n + j] != 0. {
					self.add_line(i, r, -1. * self.mat[i * n + j] / self.mat[r * n + j]);
				}
			}
			r += 1;
		}
		r
	}
}