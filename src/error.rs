use std::fmt;

#[derive(Clone, Debug)]
pub struct ErrorFactory {
	pub line: usize,
	pub file: String,
}

impl ErrorFactory {
	pub fn new(file: String, line: usize) -> ErrorFactory {
		ErrorFactory {
			line: line,
			file: file
		}
	}

	pub fn gen_error(&self, errtype: ErrorType, reason: String) -> Error {
		Error {
			errtype: errtype,
			reason: reason,
			line: self.line,
			file: self.file.clone()
		}
	}

	pub fn throw(&self, errtype: ErrorType, reason: String) -> ! {
		panic!("{}", self.gen_error(errtype, reason))
	}
}

pub struct Error {
	pub errtype: ErrorType,
	pub line: usize,
	pub file: String,
	pub reason: String,
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} on line {} in file {}! {}", self.errtype, self.line, self.file, self.reason)
	}
}

pub enum ErrorType {
	ParseError,
	RuntimeError
}

impl fmt::Display for ErrorType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", match *self {
			ErrorType::ParseError => {
				"Parsing Error"
			},
			ErrorType::RuntimeError => {
				"Runtime Error"
			}
		})
	}
}
