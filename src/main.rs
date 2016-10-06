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
use std::env;

fn run(fname: &str) {
	let path = Path::new(&fname);
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

	inter.execute(&instructions);

	println!("");
}

fn help() {
	println!(
r"              __  __       _ _   _
_/_/_/_/     |  \/  |     (_) | | |
_/       __  | \  / | __ _ _| | | |     __ _ _ __   __ _
_/_/_/  |__| | |\/| |/ _` | | | | |    / _` | '_ \ / _` |
_/           | |  | | (_| | | | | |___| (_| | | | | (_| |
_/_/_/_/     |_|  |_|\__,_|_|_| \_____/\__,_|_| |_|\__, |
                                                    __/ |
A programming language based on emails             |___/
=========================================================
See DOC.md for documentation on how to use Emailang.
Alternatively, see README.md for a quick tutorial.
emailang <file> - run the given file");
}

fn main() {
	let args = env::args().collect::<Vec<String>>();
	match args.len() {
		0 => panic!("This should not be possible!"),
		1 => help(),
		2 => run(&args[1]),
		_ => println!("Invalid number of arguments!")
	}
}
