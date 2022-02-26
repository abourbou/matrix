
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
mod ex04;
use crate::ex04::main04;
mod ex05;
use crate::ex05::main05;
mod ex06;
use crate::ex06::main06;
mod ex07;
use crate::ex07::main07;
mod ex08;
use crate::ex08::main08;
mod ex09;
use crate::ex09::main09;

fn main() {

	let mut args: Vec<String> = std::env::args().collect();

	if args.len() == 1 {
		args = String::from("null basic ex00 ex01 ex02 ex03 ex04 ex05 ex06 ex07 ex08 ex09").split(' ').map(|s| String::from(s)).collect();
			
			// [String::from(args[0].clone()), String::from("basic"), String::from("ex00"),
			// 				String::from("ex01"), String::from("ex02"), String::from("ex03"),
			// 				]);
	}

	for exercice in &args[1..] {
		match exercice.as_str() {
			"basic" => basic(),
			"ex00" => main00(),
			"ex01" => main01(),
			"ex02" => main02(),
			"ex03" => main03(),
			"ex04" => main04(),
			"ex05" => main05(),
			"ex06" => main06(),
			"ex07" => main07(),
			"ex08" => main08(),
			"ex09" => main09(),
			_	=> println!("Unknown exercice")
		}
	}
}

//TODO read doc for mod
//TODO https://www.sheshbabu.com/posts/rust-module-system/