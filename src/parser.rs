use symbols;
use symbols::Symbol;
use std::str::Chars;
use instruction::Instruction;
use user::*;
use std::slice::Iter;
use std::collections::HashMap;
use types::Type;

pub struct Parser {}

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
				if other == '\\' {
					is_esc = true;
				} else if other == m {
					if !is_esc {
						break
					}
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

pub fn take_symbols_until_semicolon<'a>(symbols: &'a mut Iter<Symbol>) -> Vec<&'a Symbol> {
	let mut ret = Vec::new();
	loop {
		let n = symbols.next();
		match n {
			None => break,
			Some(val) => {
				if let Symbol::Semicolon = *val {
					break;
				}
				ret.push(val);
			}
		}
	}
	ret
}

pub fn is_expression(symbols: &[&Symbol]) -> bool {
	for s in symbols {
		let (is_op, _) = s.get_operator();
		if is_op {
			return true;
		}
	}
	false
}

pub fn split_expression<'a>(symbols: &'a [&'a Symbol])
-> Option<(&'a [&'a Symbol], &'a Symbol, &'a [&'a Symbol])> {
	let mut pos = symbols.len();
	let mut p = 0;
	for i in 0..pos {
		let (is_op, op_prec) = symbols[i].get_operator();
		if is_op && op_prec > p {
			pos = i;
			p = op_prec;
		}
	}

	if pos == symbols.len() {
		None
	} else {
		Some((
			&symbols[..pos], &symbols[pos], &symbols[pos+1..]
		))
	}
}

impl Parser {
	pub fn new() -> Parser {
		Parser {}
	}

	fn parse_user_block(&self, block: &[Symbol]) -> HashMap<String, Vec<Instruction>> {
		let mut ret = HashMap::new();
		let mut symbols = block.iter();
		loop {
			// take one string, take one block
			let res1 = symbols.next();
			let res2 = symbols.next();
			if let None = res1 {
				if let None = res2 {
					break;
				}
			}
			let name = if let Symbol::Text(ref contents) = *res1.unwrap() {
				contents
			} else {
				panic!();
			};
			let block = if let Symbol::CurlyBraced(ref contents) = *res2.unwrap() {
				contents
			} else {
				panic!();
			};
			ret.insert(name.clone(), self.parse_symbols(&block.0));
		}
		ret
	}

	pub fn parse_string(&self, code:&str) -> Vec<Symbol> {
		let mut ret = Vec::new();
		let mut chars = code.chars();
		let mut text = String::new();
		loop {
			let c = match chars.next() {
				Some(val) => val,
				None => break
			};

			let s = match c {
				'!' => {
					Symbol::Define
				},
				',' => {
					Symbol::Comma
				},
				';' => {
					Symbol::Semicolon
				},
				'>' => {
					Symbol::Arrow
				},
				'<' => {
					let path = take_until(&mut chars, '>');
					let pos = path.find('@').unwrap();
					let (a, b) = path.split_at(pos);
					Symbol::UserPath(UserPath(a.to_string(), b[1..].to_string()))
				},
				'{' => {
					let block = take_until_matched(&mut chars, '{', '}', 0);
					Symbol::CurlyBraced(symbols::Block(self.parse_string(&block)))
				},
				'(' => {
					let block = take_until_matched(&mut chars, '(', ')', 0);
					Symbol::Parenthesis(symbols::Block(self.parse_string(&block)))
				},
				'"' => {
					Symbol::Text(take_until_unescaped(&mut chars, '"'))
				},
				'#' => {
					take_until(&mut chars, '\n');
					continue;
				},
				other => {
					if other.is_alphanumeric() || ['.', '@', '_'].contains(&other) {
						text.push(other);
					} else if !other.is_whitespace() {
						panic!("{} is not a valid character!", other);
					}
					continue;
				}
			};
			if text.len() > 0 {
				ret.push(Symbol::Identifier(text));
				text = String::new();
			}
			ret.push(s);
		}
		if text.len() > 0 {
			ret.push(Symbol::Identifier(text));
		}
		ret
	}

	pub fn parse_type(&self, symbols: &[&Symbol]) -> Type {
		if is_expression(symbols) {
			Type::Expression(Box::new(match self.parse_expression(symbols) {
				Some(val) => val,
				None => panic!("Expression is not a valid value.")
			}))
		} else if symbols.len() == 1 {
			match symbols[0].get_type() {
				Some (val) => val,
				None => panic!("Symbol is not a valid value!")
			}
		} else {
			panic!()
		}
	}

	pub fn parse_expression(&self, symbols: &[&Symbol]) -> Option<Instruction> {
		// println!("Parsing expression {:?}", symbols);
		if is_expression(symbols) {
			let (pre, mid, post) = match split_expression(symbols) {
				Some(val) => val,
				None => panic!("Not actually an expression?")
			};
			match *mid {
				Symbol::Arrow => {
					Some(Instruction::MailTo(self.parse_type(pre), self.parse_type(post)))
				},
				_ => None
			}
		} else {
			None
		}
	}

	pub fn parse_symbols(&self, symbols: &[Symbol]) -> Vec<Instruction> {
		let mut ret = Vec::new();
		let mut symbols = symbols.iter();
		loop {
			let chunk = take_symbols_until_semicolon(&mut symbols);
			if chunk.len() == 0 {
				break;
			}

			let inst = if let Symbol::Define = *chunk[0] {
				match *chunk[1] {
					Symbol::UserPath(ref path) => {
						assert!(chunk.len() <= 3);
						let block = if chunk.len() == 2 {
							HashMap::new()
						} else {
							if let Symbol::CurlyBraced(ref block) = *chunk[2] {
								self.parse_user_block(&block.0)
							} else {
								panic!()
							}
						};
						let user = User::create_user_internal(&path.0, block);
						Instruction::CreateUser(path.1.clone(), user)
					},
					Symbol::Identifier(ref name) => {
						assert!(chunk.len() <= 2);
						Instruction::CreateServer(name.clone())
					},
					_ => panic!("Unexpected Identifier")
				}
			} else {
				// expressions
				self.parse_expression(&chunk).unwrap()
			};
			ret.push(inst);
		}
		ret
	}

	pub fn parse(&self, code: &str) -> Vec<Instruction> {
		self.parse_symbols(&self.parse_string(&code))
	}
}
