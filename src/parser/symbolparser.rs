use symbols::{Symbol, SymbolDef};
use instruction::Instruction;
use instruction::CondBlock;
use user::*;
use std::slice::Iter;
use types::Type;
#[allow(unused_imports)]
use error::{SyntaxErrorFactory, SyntaxErrorType, SyntaxError};

pub fn take_symbols_until_semicolon(symbols: &mut Iter<SymbolDef>) -> Vec<SymbolDef> {
	if symbols.len() == 0 {
		return Vec::new();
	}
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

fn parse_user_block(block: &[SymbolDef]) -> Vec<(String, Vec<Instruction>)> {
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
		ret.push((name.clone(), parse_symbols(&block.0)));
	}
	ret
}

pub fn parse_ifblock(symbols: &[SymbolDef]) -> Option<Instruction> {
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
						cond: Some(parse_type(&exp)),
						block: parse_symbols(&curlybracket),
						elseblock: None
					});
				} else {
					assert!(ifblk.is_some());
					ifblk.as_mut().unwrap().append_block(CondBlock {
						cond: Some(parse_type(&exp)),
						block: parse_symbols(&curlybracket),
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
					block: parse_symbols(&curlybracket),
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

pub fn parse_type(symbols: &[SymbolDef]) -> Type {
	if is_expression(symbols) {
		Type::Expression(Box::new(match parse_expression(symbols) {
			Some(val) => val,
			None => panic!("Expression is not a valid value.")
		}))
	} else if symbols.len() == 1 {
		match symbols[0].symbol.get_type() {
			Some (val) => val,
			None => panic!("Symbol is not a valid value!")
		}
	} else {
		Type::Null
	}
}

pub fn parse_expression(symbols: &[SymbolDef]) -> Option<Instruction> {
	if is_expression(symbols) {
		let (pre, mid, post) = match split_expression(symbols) {
			Some(val) => val,
			None => panic!("Not actually an expression?")
		};
		let preval = parse_type(pre);
		let postval = parse_type(post);
		match mid.symbol {
			Symbol::Arrow => Some(Instruction::MailTo(preval, postval)),
			Symbol::Addition => Some(Instruction::Concatenate(preval, postval)),
			Symbol::Receive => {
				assert!(preval.is_null());
				Some(Instruction::GetEnv(postval))
			},
			Symbol::Slice(ref pos1, ref pos2) => {
				assert!(postval.is_null());
				Some(Instruction::Slice(preval,
					pos1.as_ref().map(|v|parse_type(&v.0)),
					pos2.as_ref().map(|v|parse_type(&v.0))))
			},
			Symbol::Index(ref pos) => {
				assert!(postval.is_null());
				Some(Instruction::Index(preval, parse_type(&pos.0)))
			},
			Symbol::Assign => {
				Some(Instruction::Assign(preval, postval))
			},
			Symbol::Modifier => {
				Some(Instruction::Modify(preval, postval))
			}
			_ => None
		}
	} else {
		None
	}
}

pub fn parse_symbols(symbols: &[SymbolDef]) -> Vec<Instruction> {
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
							parse_user_block(&block.0)
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
			parse_ifblock(&chunk).unwrap()
		} else {
			// expressions
			parse_expression(&chunk).unwrap()
		};
		ret.push(inst);
	}
	ret
}
