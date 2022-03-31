
use crate::base_struct::Vector;



/// Calcule the cosinus of the angles between two vectors
pub fn angle_cos(u: &Vector, v: &Vector) -> f32 {
	u.dot(v) / (u.norm() * v.norm())
}