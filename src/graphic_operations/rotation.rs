
use crate::*;

// Angle in degree
pub fn rx(angle : f32) -> Matrix4f
{
	let mut result = Matrix4f::identity();

	let cos_angle = angle.to_radians().cos();
	let sin_angle = angle.to_radians().sin();

	result.arr[1][1] = cos_angle;
	result.arr[2][2] = cos_angle;
	result.arr[1][2] = -1. * sin_angle;
	result.arr[2][1] = sin_angle;

	return result;
}

// Angle in degree
pub fn ry(angle : f32) -> Matrix4f
{
	let mut result = Matrix4f::identity();

	let cos_angle = angle.to_radians().cos();
	let sin_angle = angle.to_radians().sin();

	result.arr[0][0] = cos_angle;
	result.arr[2][2] = cos_angle;
	result.arr[2][0] = -1. * sin_angle;
	result.arr[0][2] = sin_angle;

	return result;
}

// Angle in degree
pub fn rz(angle : f32) -> Matrix4f
{
	let mut result = Matrix4f::identity();

	let cos_angle = angle.to_radians().cos();
	let sin_angle = angle.to_radians().sin();

	result.arr[0][0] = cos_angle;
	result.arr[1][1] = cos_angle;
	result.arr[0][1] = -1. * sin_angle;
	result.arr[1][0] = sin_angle;

	return result;
}
