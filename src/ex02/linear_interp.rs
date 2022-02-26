

pub fn lerp<T: std::ops::Mul<f32, Output = T> + std::ops::Add<T, Output = T>>(u: T, v: T, t: f32) -> T {
	let opposite = 1. - t;
	u * opposite + v * t
}