
use super::matrix::Matrix;
use super::vector::Vector;

pub fn basic()
{
	println!("Basic test for struct Matrix and Vector");
	//*vector creation
	// !let vec = Vector::from([]);
	let vec = Vector::new(0., 3);
	vec.print();
	let vec = Vector::from([0., 1., 2.]);
	vec.print();
	println!("{}", vec);
	println!("vector size : {}", vec.size());

	//*matrix creation
	// !let mat = Matrix::from([[]]);
	let mat = Matrix([[2,3,4]]);
	println!("{}", mat.0[0][0]);
	//mat.print();
	let mat = Matrix::from([[0.,0.,0.], [1.,1.,1.]]);
	//mat.print();
	//println!("{}", mat);
	//println!("matrix shape : {:?}", mat.shape());
	//println!("matrix is square : {}", mat.is_square());
}