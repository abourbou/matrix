
mod base_struct;
use crate::base_struct::{Vector, Matrix};

mod ex00;
use crate::ex00::main00::ex00;

fn basic()
{
	println!("Basic test for struct Matrix and Vector");
	//vector creation
	// !let vec = Vector::from([]);
	let vec = Vector::new(0., 3);
	vec.print();
	let vec = Vector::from([0., 1., 2.]);
	vec.print();

	//matrix creation
	// !let mat = Matrix::from([[]]);
	let mat = Matrix::new(0., 5, 5);
	mat.print();
	let mat = Matrix::from([[0.,0.,0.], [1.,1.,1.]]);
	mat.print();
	println!("matrix shape : {:?}", mat.shape());
	println!("matrix is square : {}", mat.is_square());
}

fn main() {

	let mut args: Vec<String> = std::env::args().collect();

	if args.len() == 1 {
		args = Vec::from([String::from(args[0].clone()), String::from("basic"), String::from("ex00")]);
	}

	for exercice in &args[1..] {
		match exercice.as_str() {
			"basic" => basic(),
			"ex00" => ex00(),

			_	=> println!("Unknown exercice")
		}
	}
}


//TODO add conditionnal compilation
// https://github.com/bwasty/learn-opengl-rs/blob/master/src/main.rs