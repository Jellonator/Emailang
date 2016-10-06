use instruction::Instruction;
use error::{SyntaxError};

pub mod symbolparser;
pub mod textparser;

pub fn parse(code: &str, fname: &str) -> Result<Vec<Instruction>, SyntaxError> {
	Ok(symbolparser::parse_symbols(&try!(textparser::parse_text(&code, &fname))))
}
