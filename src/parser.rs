use symbols;
use symbols::{Symbol, SymbolDef};
use std::str::Chars;
use instruction::Instruction;
use instruction::CondBlock;
use user::*;
use std::slice::Iter;
use types::Type;
use error::ErrorFactory;

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

pub fn take_symbols_until_semicolon(symbols: &mut Iter<SymbolDef>) -> Vec<SymbolDef> {
	let mut ret:Vec<SymbolDef> = Vec::new();
	loop {
		match symbols.next() {
			None => panic!(),
			Some(ref val) => {
				if let Symbol::Semicolon = val.symbol {
					break;
				}
				ret.push((*val).clone());
			}
		}
	}

	ret
}

pub fn is_expression(symbols: &[SymbolDef]) -> bool {
	for s in symbols {
		if s.symbol.get_operator().is_op() {
			return true;
		}
	}
	false
}

// what a mess of a function definition
pub fn split_expression(symbols: &[SymbolDef])
-> Option<(&[SymbolDef], &SymbolDef, &[SymbolDef])> {
	let mut pos = symbols.len();
	let mut p = 0;
	for i in 0..pos {
		let op = symbols[i].symbol.get_operator();
		if op.compare(p) {
			pos = i;
			p = op.get();
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

	fn parse_user_block(&self, block: &[SymbolDef]) -> Vec<(String, Vec<Instruction>)> {
		let mut ret = Vec::new();
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
			let name = if let Symbol::Text(ref contents) = res1.unwrap().symbol {
				contents
			} else {
				panic!("{}");
			};
			let block = if let Symbol::CurlyBraced(ref contents) = res2.unwrap().symbol {
				contents
			} else {
				panic!();
			};
			ret.push((name.clone(), self.parse_symbols(&block.0)));
		}
		ret
	}

	pub fn parse_ifblock(&self, symbols: &[SymbolDef]) -> Option<Instruction> {
		let mut blocks = Vec::new();
		let mut from_pos = 0;
		for i in 1..symbols.len() {
			match symbols[i].symbol {
				Symbol::If | Symbol::ElseIf | Symbol::Else => {
					blocks.push(&symbols[from_pos..i]);
					from_pos = i;
				}, _ => {}
			}
		}
		blocks.push(&symbols[from_pos..]);

		let mut ifblk = None;

		for block in &blocks {
			let blklen = block.len();
			match block[0].symbol {
				Symbol::If | Symbol::ElseIf => {
					let exp = &block[1..blklen-1];
					let curlybracket = match block.last().unwrap().symbol {
						Symbol::CurlyBraced(ref b) => &b.0,
						_ => panic!()
					};

					if let Symbol::If = block[0].symbol {
						assert!(ifblk.is_none());
						ifblk = Some(CondBlock {
							cond: Some(self.parse_type(&exp)),
							block: self.parse_symbols(&curlybracket),
							elseblock: None
						});
					} else {
						assert!(ifblk.is_some());
						ifblk.as_mut().unwrap().append_block(CondBlock {
							cond: Some(self.parse_type(&exp)),
							block: self.parse_symbols(&curlybracket),
							elseblock: None
						});
					}
				},
				Symbol::Else => {
					let exp = &block[1..];
					assert!(exp.len() == 1);
					let curlybracket = match block.last().unwrap().symbol {
						Symbol::CurlyBraced(ref b) => &b.0,
						_ => panic!()
					};
					assert!(ifblk.is_some());
					ifblk.as_mut().unwrap().append_block(CondBlock {
						cond: None,
						block: self.parse_symbols(&curlybracket),
						elseblock: None
					});
				},
				_ => panic!()
			}
		}

		match ifblk {
			Some(val) => Some(Instruction::IfBlock(val)),
			None => None
		}
	}

	pub fn parse_string(&self, code: &str, fname: &str) -> Vec<SymbolDef> {
		let mut ret = Vec::new();
		let mut chars = code.chars();
		let mut text = String::new();
		let mut line = 1;
		loop {
			let c = match chars.next() {
				Some(val) => val,
				None => break
			};

			let s = match c {
				'!' => Symbol::Define,
				',' => Symbol::Comma,
				';' => Symbol::Semicolon,
				'>' => Symbol::Arrow,
				'+' => Symbol::Addition,
				'@' => Symbol::Receive,
				'=' => Symbol::Assign,
				'<' => {
					let path = take_until(&mut chars, '>');
					let pos = path.find('@').unwrap();
					let (a, b) = path.split_at(pos);
					Symbol::UserPath(UserPath(a.to_string(), b[1..].to_string()))
				},
				'{' => {
					let block = take_until_matched(&mut chars, '{', '}', 0);
					Symbol::CurlyBraced(symbols::Block(self.parse_string(&block, fname)))
				},
				'(' => {
					let block = take_until_matched(&mut chars, '(', ')', 0);
					Symbol::Parenthesis(symbols::Block(self.parse_string(&block, fname)))
				},
				'"' => {
					Symbol::Text(take_until_unescaped(&mut chars, '"'))
				},
				'[' => {
					let indexcontents = take_until(&mut chars, ']');
					match indexcontents.find(':') {
						None => {
							Symbol::Index(indexcontents.parse::<isize>().unwrap())
						},
						Some(pos) => {
							let val1 = &indexcontents[..pos];
							let val2 = &indexcontents[pos+1..];
							Symbol::Slice(
								Some(val1.parse::<isize>().unwrap()),
								Some(val2.parse::<isize>().unwrap())
							)
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
					} else if other.is_alphanumeric() || ['.', '@', '_'].contains(&other) {
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
					errfactory: ErrorFactory {
						line: line,
						file: fname.to_string()
					}
				});
				text = String::new();
			}
			ret.push(SymbolDef{
				symbol: s,
				errfactory: ErrorFactory {
					line: line,
					file: fname.to_string()
				}
			});
		}
		if text.len() > 0 {
			ret.push(SymbolDef{
				symbol: Symbol::Identifier(text),
				errfactory: ErrorFactory {
					line: line,
					file: fname.to_string()
				}
			});
		}
		ret
	}

	pub fn parse_type(&self, symbols: &[SymbolDef]) -> Type {
		if is_expression(symbols) {
			Type::Expression(Box::new(match self.parse_expression(symbols) {
				Some(val) => val,
				None => panic!("Expression is not a valid value.")
			}))
		} else if symbols.len() == 1 {
			match symbols[0].symbol.get_type(self) {
				Some (val) => val,
				None => panic!("Symbol is not a valid value!")
			}
		} else {
			Type::Null
		}
	}

	pub fn parse_expression(&self, symbols: &[SymbolDef]) -> Option<Instruction> {
		if is_expression(symbols) {
			let (pre, mid, post) = match split_expression(symbols) {
				Some(val) => val,
				None => panic!("Not actually an expression?")
			};
			let preval = self.parse_type(pre);
			let postval = self.parse_type(post);
			match mid.symbol {
				Symbol::Arrow => Some(Instruction::MailTo(preval, postval)),
				Symbol::Addition => Some(Instruction::Concatenate(preval, postval)),
				Symbol::Receive => {
					assert!(preval.is_null());
					Some(Instruction::GetEnv(postval))
				},
				Symbol::Slice(pos1, pos2) => {
					assert!(postval.is_null());
					Some(Instruction::Slice(preval, pos1, pos2))
				},
				Symbol::Index(pos) => {
					assert!(postval.is_null());
					Some(Instruction::Index(preval, pos))
				},
				Symbol::Assign => {
					Some(Instruction::Assign(preval, postval))
				},
				_ => None
			}
		} else {
			None
		}
	}

	pub fn parse_symbols(&self, symbols: &[SymbolDef]) -> Vec<Instruction> {
		let mut ret = Vec::new();
		let mut symbols = symbols.iter();
		loop {
			let chunk = take_symbols_until_semicolon(&mut symbols);
			if chunk.len() == 0 {
				break;
			}

			let inst = if let Symbol::Define = chunk[0].symbol {
				match chunk[1].symbol {
					Symbol::UserPath(ref path) => {
						assert!(chunk.len() <= 3);
						let block = if chunk.len() == 2 {
							Vec::new()
						} else {
							if let Symbol::CurlyBraced(ref block) = chunk[2].symbol {
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
			} else if let Symbol::If = chunk[0].symbol {
				self.parse_ifblock(&chunk).unwrap()
			} else {
				// expressions
				self.parse_expression(&chunk).unwrap()
			};
			ret.push(inst);
		}
		ret
	}

	pub fn parse(&self, code: &str, fname: &str) -> Vec<Instruction> {
		self.parse_symbols(&self.parse_string(&code, &fname))
	}
}
