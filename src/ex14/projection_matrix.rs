
use crate::base_struct::Matrix;

/// compute a projection matrix
///
/// compute a projection matrix to transform a 3D item of opengl into a 2D screen
///
/// src: <https://www.youtube.com/watch?v=EqNcqBdrNyI>
pub fn proj_mat(fov: f32, ratio: f32, near: f32, far: f32) -> Matrix {
	let mut proj_mat : Matrix = Matrix::new(0., 4, 4);
	let lambda = far / (far - near);

	proj_mat.mat[0] = ratio * fov;
	proj_mat.mat[5] = fov;
	proj_mat.mat[10] = lambda;
	proj_mat.mat[11] = -1. * lambda * near;
	proj_mat.mat[15] = 1.;

	proj_mat
}