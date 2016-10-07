use instruction::Instruction;
use error::{SyntaxError};

pub mod symbolparser;
pub mod textparser;

pub fn parse(code: &str) -> Result<Vec<Instruction>, SyntaxError> {
	symbolparser::parse_symbols(&try!(textparser::parse_text(&code)))
}
