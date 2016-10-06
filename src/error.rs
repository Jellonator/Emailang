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
			pos: Some((self.line, self.column))
		}
	}

	// pub fn throw(&self, errortype: SyntaxErrorType) -> ! {
	// 	panic!("{}", self.gen_error(errortype))
	// }
	//
	// pub fn throw_new(&self, errortype: SyntaxErrorType) -> ! {
	// 	panic!("{}", self.gen_error(errortype))
	// }
}

pub enum SyntaxErrorType {
	UnexpectedSymbol(char),
	ExpectedSemicolon,
	MalformedUserpath,
	MalformedIfStatement,
	NotAType,
	BadExpression,
}

pub struct SyntaxError {
	pub pos: Option<(usize, usize)>,
	pub errortype: SyntaxErrorType
}

impl SyntaxError {
	pub fn new(line: usize, column: usize, errortype: SyntaxErrorType) -> SyntaxError {
		SyntaxError {
			errortype: errortype,
			pos: Some((line, column))
		}
	}
	pub fn new_eof(errortype: SyntaxErrorType) -> SyntaxError {
		SyntaxError {
			errortype: errortype,
			pos: None
		}
	}
}

impl fmt::Display for SyntaxError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Syntax Error {}! {}.", match self.pos {
			Some(pos) => format!("{}:{}", pos.0, pos.1),
			None => "at end of file".to_string()
		}, self.errortype)
	}
}

impl fmt::Display for SyntaxErrorType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			SyntaxErrorType::UnexpectedSymbol(ref c) => write!(f, "Unexpected character '{}'", c),
			SyntaxErrorType::ExpectedSemicolon => write!(f, "Expected semicolon"),
			SyntaxErrorType::MalformedUserpath => write!(f, "Malformed Userpath"),
			SyntaxErrorType::MalformedIfStatement => write!(f, "Malformed if statement"),
			SyntaxErrorType::NotAType => write!(f, "Not a type"),
			SyntaxErrorType::BadExpression => write!(f, "Bad expression"),
		}
	}
}
