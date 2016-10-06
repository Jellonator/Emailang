use std::fmt;

#[derive(Clone, Debug)]
pub struct SyntaxErrorFactory {
	pub line: usize,
	pub column: usize,
}

impl SyntaxErrorFactory {
	pub fn new(line: usize, column: usize) -> SyntaxErrorFactory {
		SyntaxErrorFactory {
			column: column,
			line: line,
		}
	}

	pub fn gen_error(&self, errortype: SyntaxErrorType) -> SyntaxError {
		SyntaxError {
			errortype: errortype,
			line: self.line,
			column: self.column
		}
	}

	pub fn throw(&self, errortype: SyntaxErrorType) -> ! {
		panic!("{}", self.gen_error(errortype))
	}
}

pub enum SyntaxErrorType {
	UnexpectedSymbol(char),
	ExpectedSemicolon,
	MalformedUserpath,
}

pub struct SyntaxError {
	pub line: usize,
	pub column: usize,
	pub errortype: SyntaxErrorType
}

impl fmt::Display for SyntaxError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Syntax Error on line {}:{}! {}.", self.line, self.column, self.errortype)
	}
}

impl fmt::Display for SyntaxErrorType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			SyntaxErrorType::UnexpectedSymbol(ref c) => write!(f, "Unexpected character '{}'", c),
			SyntaxErrorType::ExpectedSemicolon => write!(f, "Expected semicolon"),
			SyntaxErrorType::MalformedUserpath => write!(f, "Malformed Userpath")
		}
	}
}
