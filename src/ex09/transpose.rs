
use crate::base_struct::Matrix;

impl Matrix {
	pub fn transpose(&self) -> Matrix {
		let mut mat :Vec<f32> = Vec::new();
		let [m,n] = self.shape();
		for j in 0..n {
			for i in 0..m {
				mat.push(self.mat[i * n + j]);
			}
		}
		Matrix {mat: mat, number_rows: n, number_cols: m}
	}
}