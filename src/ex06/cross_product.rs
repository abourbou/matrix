
use crate::base_struct::Vector;

pub fn cross_product(u: &Vector, v: &Vector) -> Vector
{
	Vector::from([	u.vec[1] * v.vec[2] - u.vec[2] * v.vec[1],
					u.vec[2] * v.vec[0] - u.vec[0] * v.vec[2],
					u.vec[0] * v.vec[1] - u.vec[1] * v.vec[0]])
}