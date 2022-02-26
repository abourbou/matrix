
use crate::base_struct::Matrix;

pub fn main09() {
	let mat = Matrix::from([[0.,1.,2.],[3.,4.,5.],[6.,7.,8.]]);
	println!("original matrix : \n{}", mat);
	println!("transpose matrix : \n{}", mat.transpose());
	let mat = Matrix::from([[0.,1.,2.],[3.,4.,5.]]);
	println!("original matrix : \n{}", mat);
	println!("transpose matrix : \n{}", mat.transpose());
	let mat = Matrix::from([[0.,1.],[2.,3.],[4.,5.]]);
	println!("original matrix : \n{}", mat);
	println!("transpose matrix : \n{}", mat.transpose());
}