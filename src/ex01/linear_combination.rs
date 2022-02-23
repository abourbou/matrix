
use crate::base_struct::Vector;

pub fn linear_combination(u: &[Vector], coefs: &[f32]) -> Vector {
	if u.len() == 0 {
		panic!("empty list of vector");
	}
	else if u.len() != coefs.len() {
		panic!("list coeff need to be the same size that the list of vector");
	}

	let size_vector = u[0].size();
	for i in 0..u.len()
	{
		if u[i].size() != size_vector {
			panic!("vector have inconsistent size");
		}
	}

	let mut vec_lincb : Vec<f32> = Vec::new();
	for i in  0..size_vector {
		let mut buffer_value : f32 = 0.;
		for j in 0..u.len() {
			buffer_value += u[j].vec[i] * coefs[j];
		}
		vec_lincb.push(buffer_value);
	}
	Vector {vec: vec_lincb, size: size_vector}
}