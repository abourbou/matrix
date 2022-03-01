use super::proj_mat;

pub fn main14() {
	let mat = proj_mat(90., 16./9., 10., 30.);
	println!("mat proj : \n{}", mat);
	let mat = proj_mat(180., 16./9., 1., 2.);
	println!("mat proj : \n{}", mat);

}