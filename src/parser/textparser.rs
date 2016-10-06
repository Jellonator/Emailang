use symbols;
use symbols::{Symbol, SymbolDef};
use std::str::Chars;
use user::*;
use error::{SyntaxErrorFactory, SyntaxError, SyntaxErrorType};

pub fn take_until(chars: &mut Chars, c: char) -> String {
	let mut ret = String::new();
	loop {
		let n = chars.next();
		match n {
			None => break,
			Some(val) => {
				if val == c {
					break
				}
				ret.push(val);
			}
		}
	}
	ret
}

pub fn take_until_unescaped(chars: &mut Chars, m: char) -> String {
	let mut ret = String::new();
	let mut is_esc = false;
	loop {
		let n = chars.next();
		match n {
			None => break,
			Some(other) => {
				if other == '\\' && is_esc {
					is_esc = true;
				} else {
					if other == m {
						if !is_esc {
							break
						}
					}
					is_esc = false;
				}
				ret.push(other);
			}
		}
	}
	ret
}

pub fn take_until_matched(chars: &mut Chars, begin: char, end: char, target_level: i32) -> String {
	let mut ret = String::new();
	let mut level: i32 = 1;
	loop {
		let n = chars.next();
		match n {
			None => break,
			Some(val) => {
				if val == begin {
					level += 1;
				}
				if val == end {
					level -= 1;
				}
				if level == target_level {
					break;
				}
				ret.push(val);
			}
		}
	}
	ret
}

pub fn parse_text(code: &str, fname: &str) -> Result<Vec<SymbolDef>, SyntaxError> {
	let mut ret = Vec::new();
	let mut chars = code.chars();
	let mut text = String::new();
	let mut line = 1;
	let mut column = 0;
	loop {
		let c = match chars.next() {
			Some(val) => val,
			None => break
		};
		column += 1;
		let s = match c {
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
				let pos = try!(path.find('@').ok_or(SyntaxError::new(line, column, SyntaxErrorType::MalformedUserpath)));
				let (a, b) = path.split_at(pos);
				Symbol::UserPath(UserPath(a.to_string(), b[1..].to_string()))
			},
			'{' => {
				let block = take_until_matched(&mut chars, '{', '}', 0);
				Symbol::CurlyBraced(symbols::Block(try!(parse_text(&block, fname))))
			},
			'(' => {
				let block = take_until_matched(&mut chars, '(', ')', 0);
				Symbol::Parenthesis(symbols::Block(try!(parse_text(&block, fname))))
			},
			'"' => {
				Symbol::Text(take_until_unescaped(&mut chars, '"'))
			},
			'[' => {
				let indexcontents = take_until(&mut chars, ']');
				match indexcontents.find(':') {
					None => {
						Symbol::Index(symbols::Block(
							try!(parse_text(&indexcontents, fname))))
					},
					Some(pos) => {
						let val1 = &indexcontents[..pos];
						let val1 = match val1.trim() {
							"" => None,
							other => Some(symbols::Block(try!(parse_text(&other, fname))))
						};
						let val2 = &indexcontents[pos+1..];
						let val2 = match val2.trim() {
							"" => None,
							other => Some(symbols::Block(try!(parse_text(&other, fname))))
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
				if other == '\n' {
					line += 1;
					column = 0;
				} else if other.is_alphanumeric() || ['.', '@', '_', '-'].contains(&other) {
					text.push(other);
				} else if !other.is_whitespace() {
					panic!("{} is not a valid character!", other);
				}
				continue;
			}
		};
		if text.len() > 0 {
			ret.push(SymbolDef{
				symbol: match text.as_str() {
					"if" => Symbol::If,
					"else" => Symbol::Else,
					"elif" => Symbol::ElseIf,
					other => Symbol::Identifier(other.to_string()),
				},
				errfactory: SyntaxErrorFactory::new(line, column)
			});
			text = String::new();
		}
		ret.push(SymbolDef{
			symbol: s,
			errfactory: SyntaxErrorFactory::new(line, column)
		});
	}
	if text.len() > 0 {
		ret.push(SymbolDef{
			symbol: Symbol::Identifier(text),
			errfactory: SyntaxErrorFactory::new(line, column)
		});
	}
	Ok(ret)
}
