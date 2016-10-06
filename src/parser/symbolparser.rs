use symbols::{Symbol, SymbolDef};
use instruction::Instruction;
use instruction::CondBlock;
use user::*;
use types::Type;
#[allow(unused_imports)]
use error::{SyntaxErrorFactory, SyntaxErrorType, SyntaxError};

pub fn split_semicolon(symbols: &[SymbolDef]) -> Result<Vec<Vec<SymbolDef>>,SyntaxError> {
	if symbols.len() == 0 {
		// empty blocks are okay
		return Ok(Vec::new());
	}
	// Make sure ends with semicolon(unwrap here is fine)
	if let Symbol::Semicolon = symbols.last().unwrap().symbol {}
	else {
		return Err(symbols.last().unwrap().errfactory.gen_error(SyntaxErrorType::ExpectedSemicolon));
	}
	// Split at each semicolon
	Ok(symbols.split(
		|v| if let Symbol::Semicolon = v.symbol {true} else {false}
	).map(|v|v.to_vec()).collect())
}

pub fn is_expression(symbols: &[SymbolDef]) -> bool {
	for s in symbols {
		if s.get_operator().is_op() {
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
		let op = symbols[i].get_operator();
		if op.compare(p) {
			pos = i;
			p = op.get();
		}
	}

	if pos == symbols.len() {
		None
	} else {
		Some((&symbols[..pos], &symbols[pos], &symbols[pos+1..]))
	}
}

fn parse_user_block(block: &[SymbolDef]) -> Result<Vec<(String, Vec<Instruction>)>, SyntaxError> {
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
		ret.push((name.clone(), try!(parse_symbols(&block.0))));
	}
	Ok(ret)
}

pub fn parse_ifblock(symbols: &[SymbolDef]) -> Result<Instruction, SyntaxError> {
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
						cond: Some(try!(parse_type(&exp))),
						block: try!(parse_symbols(&curlybracket)),
						elseblock: None
					});
				} else {
					assert!(ifblk.is_some());
					ifblk.as_mut().unwrap().append_block(CondBlock {
						cond: Some(try!(parse_type(&exp))),
						block: try!(parse_symbols(&curlybracket)),
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
					block: try!(parse_symbols(&curlybracket)),
					elseblock: None
				});
			},
			_ => panic!()
		}
	}

	ifblk.map(|val|Instruction::IfBlock(val))
	.ok_or(symbols[0].errfactory.gen_error(SyntaxErrorType::MalformedIfStatement))
}

pub fn parse_type(symbols: &[SymbolDef]) -> Result<Type, SyntaxError> {
	Ok(if is_expression(symbols) {
		Type::Expression(Box::new(try!(parse_expression(symbols))))
	} else if symbols.len() == 1 {
		try!(symbols[0].get_type())
	} else {
		Type::Null
	})
}

pub fn parse_expression(symbols: &[SymbolDef]) -> Result<Instruction, SyntaxError> {
	if is_expression(symbols) {
		let (pre, mid, post) = match split_expression(symbols) {
			Some(val) => val,
			None => panic!("This shouldn't happen! Not actually an expression?")
		};
		let preval = try!(parse_type(pre));
		let postval = try!(parse_type(post));
		match mid.symbol {
			Symbol::Arrow => Ok(Instruction::MailTo(preval, postval)),
			Symbol::Addition => Ok(Instruction::Concatenate(preval, postval)),
			Symbol::Receive => {
				assert!(preval.is_null());
				Ok(Instruction::GetEnv(postval))
			},
			Symbol::Slice(ref pos1, ref pos2) => {
				assert!(postval.is_null());
				Ok(Instruction::Slice(preval,
					match *pos1 {
						Some(ref val) => Some(try!(parse_type(&val.0))),
						None => None
					},
					match *pos2 {
						Some(ref val) => Some(try!(parse_type(&val.0))),
						None => None
					}
				))
			},
			Symbol::Index(ref pos) => {
				assert!(postval.is_null());
				Ok(Instruction::Index(preval, try!(parse_type(&pos.0))))
			},
			Symbol::Assign => {
				Ok(Instruction::Assign(preval, postval))
			},
			Symbol::Modifier => {
				Ok(Instruction::Modify(preval, postval))
			}
			_ => Err(symbols[0].errfactory.gen_error(SyntaxErrorType::BadExpression))
		}
	} else {
		Err(symbols[0].errfactory.gen_error(SyntaxErrorType::BadExpression))
	}
}

pub fn parse_symbols(symbols: &[SymbolDef]) -> Result<Vec<Instruction>, SyntaxError> {
	let mut ret = Vec::new();
	for chunk in try!(split_semicolon(symbols)) {
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
							try!(parse_user_block(&block.0))
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
			try!(parse_ifblock(&chunk))
		} else {
			// expressions
			try!(parse_expression(&chunk))
		};
		ret.push(inst);
	}
	Ok(ret)
}
