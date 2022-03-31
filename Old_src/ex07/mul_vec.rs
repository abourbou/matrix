
use crate::base_struct::{Vector, Matrix};

impl Matrix {
	pub fn mul_vec(&self, vec: &Vector) -> Vector {
		let [m, n] = self.shape();
		if vec.size() != n {
			panic!("unvalid size for matrix multiplication with a vector");
		}
		let mut new_vec : Vec<f32> = Vec::new();
		for i in 0..m {
			new_vec.push(self.mat[i*n..(i+1)*n].iter().zip(vec.vec.iter()).map(|(m, v)| m * v).sum());
		}
		Vector {vec: new_vec, size: m}
	}

	pub fn mul_mat(&self, mat: &Matrix) -> Matrix {
		let [m, n] = self.shape();
		let [n2, p] = mat.shape();
		if n2 != n {
			panic!("unvalid size for matrix multiplication with a matrix");
		}
		println!("m: {}, n: {}, p: {}", m, n, p);
		let mut new_mat : Vec<f32> = Vec::new();
		for i in 0..m {
			for j in 0..p {
				new_mat.push(self.mat[i*n..(i+1)*n].iter()
				.zip(mat.mat.iter().enumerate().filter(|(it, _x)| it % p == j).map(|(_it, x)| x))
				.map(|(&s, &rhs)| s * rhs).sum()
			)
			}
		}
		Matrix {mat: new_mat, number_rows: m, number_cols: p}
	}
}