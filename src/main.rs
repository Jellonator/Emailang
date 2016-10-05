pub mod types;
pub mod symbols;
pub mod interpreter;
pub mod parser;
pub mod instruction;
pub mod user;
pub mod server;
pub mod mail;
pub mod error;
pub mod environment;
extern crate regex;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
	let path = Path::new("main.email");
	let display = path.display();

	let mut file = match File::open(path) {
		Ok(val) => val,
		Err(err) => panic!("couldn't open {}: {}", display, err.description())
	};

	let mut contents = String::new();
	match file.read_to_string(&mut contents) {
		Err(why) => panic!("couldn't read {}: {}", display, why.description()),
		Ok(_) => {}
	};

	let p = parser::Parser::new();
	let symbols = p.parse_string(&contents, path.to_str().unwrap());

	let instructions = p.parse_symbols(&symbols);
	let mut inter = interpreter::Interpreter::new();

	inter.run(&instructions, &mut environment::Environment::new_anon());
}
