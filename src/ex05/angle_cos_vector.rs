
use crate::base_struct::Vector;

pub fn angle_cos(u: &Vector, v: &Vector) -> f32 {
	u.dot(v) / (u.norm() * v.norm())
}