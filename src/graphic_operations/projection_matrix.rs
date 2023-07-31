
use crate::base::{matrix::Matrix};

/// compute a projection matrix
///
/// compute a projection matrix to transform a 3D item of opengl into a 2D screen
///
pub fn proj_mat(fov: f32, screen_ratio: f32, z_near: f32, z_far: f32) -> Matrix<f32,4,4> {
	let mut proj_mat = Matrix::<f32,4,4>::new(0.);
	let tan_half_fov = (fov * std::f32::consts::PI / 360.).tan();

	proj_mat.arr[0][0] = 1. / (screen_ratio * tan_half_fov);
	proj_mat.arr[1][1] = 1. / tan_half_fov;
	proj_mat.arr[2][2] = - (z_far + z_near) / (z_far - z_near);
	proj_mat.arr[3][2] = -1.;
	proj_mat.arr[2][3] = (-2. * z_far * z_near) / (z_far - z_near);

	proj_mat
}

#[cfg(test)]
	#[test]
	fn test00() {
		assert_eq!(proj_mat(90., 4./3., 0.01, 100.), Matrix::from([
			[0.75, 0.0, 0.0, 0.0],
			[0.0, 1.0, 0.0, 0.0],
			[0.0, 0.0, -1.0002, -0.020002],
			[0.0, 0.0, -1.0, 0.0]
		]));
	}
	#[test]
	fn test01() {
		assert_eq!(proj_mat(70., 16./9., 1., 1000.), Matrix::from([
			[0.80333316, 0.0, 0.0, 0.0],
			[0.0, 1.4281479, 0.0, 0.0],
			[0.0, 0.0, -1.002002, -2.002002],
			[0.0, 0.0, -1.0, 0.0]
		]));
	}
