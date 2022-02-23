
mod base_struct;
use crate::base_struct::basic;

mod ex00;
use crate::ex00::main00;

mod ex01;
use crate::ex01::main01;

fn main() {

	let mut args: Vec<String> = std::env::args().collect();

	if args.len() == 1 {
		args = Vec::from([String::from(args[0].clone()), String::from("basic"), String::from("ex00")]);
	}

	for exercice in &args[1..] {
		match exercice.as_str() {
			"basic" => basic(),
			"ex00" => main00(),
			"ex01" => main01(),
			_	=> println!("Unknown exercice")
		}
	}
}

//TODO read doc for mod
//TODO https://www.sheshbabu.com/posts/rust-module-system/