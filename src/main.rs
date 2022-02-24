
mod base_struct;
use crate::base_struct::basic;

mod ex00;
use crate::ex00::main00;

mod ex01;
use crate::ex01::main01;

mod ex02;
use crate::ex02::main02;

mod ex03;
use crate::ex03::main03;

fn main() {

	let mut args: Vec<String> = std::env::args().collect();

	if args.len() == 1 {
		args = Vec::from([String::from(args[0].clone()), String::from("basic"), String::from("ex00"),
							String::from("ex01"), String::from("ex02"), String::from("ex03"),
							]);
	}

	for exercice in &args[1..] {
		match exercice.as_str() {
			"basic" => basic(),
			"ex00" => main00(),
			"ex01" => main01(),
			"ex02" => main02(),
			"ex03" => main03(),
			_	=> println!("Unknown exercice")
		}
	}
}

//TODO read doc for mod
//TODO https://www.sheshbabu.com/posts/rust-module-system/