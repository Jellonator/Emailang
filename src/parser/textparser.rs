use symbols;
use symbols::{Symbol, SymbolDef};
use user::*;
use std::slice::Iter;
use error::{SyntaxErrorFactory, SyntaxError, SyntaxErrorType};
use std::fmt;

#[derive(Clone, Copy)]
pub struct CodeChar {
	pub val: char,
	pub line: usize,
	pub column: usize
}

impl fmt::Display for CodeChar {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.val.to_string())
	}
}

pub fn codechars_to_string(chars: &[CodeChar]) -> String {
	chars.iter().map(|v|v.val).collect()
}

pub fn codechars_find(chars: &[CodeChar], c: char) -> Option<usize> {
	chars.iter().position(|v|v.val == c)
}

pub fn codechars_trimmed(chars: &[CodeChar]) -> String {
	codechars_to_string(chars).trim().to_string()
}

pub fn take_until(chars: &mut Iter<CodeChar>, c: char) -> Vec<CodeChar> {
	let mut ret = Vec::new();
	loop {
		let n = chars.next();
		match n {
			None => break,
			Some(val) => {
				if val.val == c {
					break
				}
				ret.push(*val);
			}
		}
	}
	ret
}

pub fn take_until_unescaped(chars: &mut Iter<CodeChar>, m: char) -> Vec<CodeChar> {
	let mut ret = Vec::new();
	let mut is_esc = false;
	loop {
		let n = chars.next();
		match n {
			None => break,
			Some(other) => {
				if other.val == '\\' && is_esc {
					is_esc = true;
				} else {
					if other.val == m {
						if !is_esc {
							break
						}
					}
					is_esc = false;
				}
				ret.push(*other);
			}
		}
	}
	ret
}

pub fn take_until_matched(chars: &mut Iter<CodeChar>, begin: char, end: char, target_level: i32) -> Vec<CodeChar> {
	let mut ret = Vec::new();
	let mut level: i32 = 1;
	loop {
		let n = chars.next();
		match n {
			None => break,
			Some(val) => {
				if val.val == begin {
					level += 1;
				}
				if val.val == end {
					level -= 1;
				}
				if level == target_level {
					break;
				}
				ret.push(*val);
			}
		}
	}
	ret
}

pub fn parse_text(code: &str) -> Result<Vec<SymbolDef>, SyntaxError> {
	let mut line = 1;
	let mut column = 0;
	parse_code(&code.chars().map(|v| {
		if v == '\n' {
			line += 1;
			column = 0;
		}
		column += 1;
		CodeChar {
			val: v,
			line: line,
			column: column
		}
	}).collect::<Vec<CodeChar>>())
}

pub fn take_identifier(chars: &mut Vec<SymbolDef>, text: &mut String, c: &CodeChar) {
	if text.len() > 0 {
		chars.push(SymbolDef {
			symbol: match text.as_str() {
				"if" => Symbol::If,
				"else" => Symbol::Else,
				"elif" => Symbol::ElseIf,
				other => Symbol::Identifier(other.to_string()),
			},
			errfactory: SyntaxErrorFactory::new(c.line, c.column)
		});
		text.clear();
	}
}

pub fn parse_code(code: &[CodeChar]) -> Result<Vec<SymbolDef>, SyntaxError> {
	let mut ret = Vec::new();
	let mut chars = code.iter();
	let mut text = String::new();
	let mut lastchar = None;
	loop {
		let c = match chars.next() {
			Some(val) => val,
			None => break
		};
		lastchar = Some(c.clone());
		let s = match c.val {
			'!' => Symbol::Define,
			',' => Symbol::Comma,
			';' => Symbol::Semicolon,
			'>' => Symbol::Arrow,
			'+' => Symbol::Addition,
			'@' => Symbol::Receive,
			'=' => Symbol::Assign,
			'|' => Symbol::Modifier,
			'<' => {
				let path = take_until(&mut chars, '>');
				let pos = try!(codechars_find(&path, '@').ok_or(SyntaxError::new(
					c.line, c.column, SyntaxErrorType::MalformedUserpath)));
				let (a, b) = path.split_at(pos);
				Symbol::UserPath(UserPath(codechars_to_string(a),
					codechars_to_string(&b[1..])))
			},
			'{' => {
				let block = take_until_matched(&mut chars, '{', '}', 0);
				Symbol::CurlyBraced(symbols::Block(try!(parse_code(&block))))
			},
			'(' => {
				let block = take_until_matched(&mut chars, '(', ')', 0);
				Symbol::Parenthesis(symbols::Block(try!(parse_code(&block))))
			},
			'"' => {
				Symbol::Text(codechars_to_string(&take_until_unescaped(&mut chars, '"')))
			},
			'[' => {
				let indexcontents = take_until(&mut chars, ']');
				match codechars_find(&indexcontents, ':') {
					None => {
						Symbol::Index(symbols::Block(
							try!(parse_code(&indexcontents))))
					},
					Some(pos) => {
						let val1 = &indexcontents[..pos];
						let val1 = match codechars_trimmed(&val1).as_str() {
							"" => None,
							_ => Some(symbols::Block(try!(parse_code(val1))))
						};
						let val2 = &indexcontents[pos+1..];
						let val2 = match codechars_trimmed(&val2).as_str() {
							"" => None,
							_ => Some(symbols::Block(try!(parse_code(val2))))
						};
						Symbol::Slice(val1, val2)
					}
				}
			},
			'#' => {
				take_until(&mut chars, '\n');
				continue;
			},
			other => {
				if other.is_alphanumeric() || ['.', '@', '_', '-'].contains(&other) {
					text.push(other);
				} else if other.is_whitespace() {
					take_identifier(&mut ret, &mut text, &c);
				} else {
					panic!("{} is not a valid character!", other);
				}
				continue;
			}
		};
		take_identifier(&mut ret, &mut text, &c);
		ret.push(SymbolDef{
			symbol: s,
			errfactory: SyntaxErrorFactory::new(c.line, c.column)
		});
	}
	if let Some(ref lastc) = lastchar {
		take_identifier(&mut ret, &mut text, &lastc);
	}
	Ok(ret)
}
