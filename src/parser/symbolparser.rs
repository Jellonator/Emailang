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
pub fn split_expression(symbols: &[SymbolDef], errfact: SyntaxErrorFactory)
-> Result<(&[SymbolDef], &SymbolDef, &[SymbolDef]), SyntaxError> {
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
		Err(errfact.gen_error(SyntaxErrorType::BadExpression))
	} else {
		Ok((&symbols[..pos], &symbols[pos], &symbols[pos+1..]))
	}
}

fn parse_user_block(block: &[SymbolDef]) -> Result<Vec<(String, Vec<Instruction>)>, SyntaxError> {
	let mut ret = Vec::new();
	let errfact = SyntaxErrorFactory::from_symbols(block);
	for chunk in try!(split_semicolon(block)) {
		if chunk.len() == 0 {
			continue;
		}
		if chunk.len() != 2 {
			return Err(errfact.gen_error(SyntaxErrorType::BadUserBlock))
		}
		let name = if let Symbol::Text(ref contents) = chunk[0].symbol {
			contents
		} else {
			return Err(errfact.gen_error(SyntaxErrorType::BadUserBlock))
		};
		let block = if let Symbol::CurlyBraced(ref contents) = chunk[1].symbol {
			contents
		} else {
			return Err(errfact.gen_error(SyntaxErrorType::BadUserBlock))
		};
		ret.push((name.clone(), try!(parse_symbols(&block.0))));
	}
	Ok(ret)
}

// TODO: rewrite this monstrosity!
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
		Type::Expression(Box::new(
			try!(parse_expression(symbols, SyntaxErrorFactory::from_symbols(symbols)))))
	} else if symbols.len() == 1 {
		try!(symbols[0].get_type())
	} else {
		Type::Null
	})
}

pub fn parse_expression(symbols: &[SymbolDef], errfact: SyntaxErrorFactory) -> Result<Instruction, SyntaxError> {
	let (pre, mid, post) = try!(split_expression(symbols, errfact.clone()));
	let preval = try!(parse_type(pre));
	let postval = try!(parse_type(post));
	if preval.is_null() == mid.get_operator().preval() {
		return Err(mid.errfactory.gen_error(SyntaxErrorType::BadExpression));
	}
	if postval.is_null() == mid.get_operator().postval() {
		return Err(mid.errfactory.gen_error(SyntaxErrorType::BadExpression));
	}
	match mid.symbol {
		Symbol::Arrow => Ok(Instruction::MailTo(preval, postval)),
		Symbol::Addition => Ok(Instruction::Concatenate(preval, postval)),
		Symbol::Assign => Ok(Instruction::Assign(preval, postval)),
		Symbol::Modifier => Ok(Instruction::Modify(preval, postval)),
		Symbol::Receive => Ok(Instruction::GetEnv(postval)),
		Symbol::Index(ref pos) => Ok(Instruction::Index(preval, try!(parse_type(&pos.0)))),
		Symbol::Slice(ref pos1, ref pos2) => {
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
		_ => Err(symbols[0].errfactory.gen_error(SyntaxErrorType::BadExpression))
	}
}

pub fn parse_symbols(symbols: &[SymbolDef]) -> Result<Vec<Instruction>, SyntaxError> {
	let mut ret = Vec::new();
	for chunk in try!(split_semicolon(symbols)) {
		if chunk.len() == 0 {
			continue;
		}

		let inst = if let Symbol::Define = chunk[0].symbol {
			assert!(chunk.len() >= 2);
			let ref symbol = chunk[1].symbol;
			let chunk = &chunk[1..];
			match *symbol {
				Symbol::UserPath(ref name, ref server) => {
					let block = match chunk.len() {
						1 => Vec::new(),
						2 => {
							if let Symbol::CurlyBraced(ref block) = chunk[1].symbol {
								try!(parse_user_block(&block.0))
							} else {
								return Err(chunk[1].errfactory.gen_error(
									SyntaxErrorType::BadUserBlock))
							}
						},
						_ => return Err(chunk[1].errfactory.gen_error(
							SyntaxErrorType::BadUserBlock))
					};
					let user = UserDef::create_def_internal(block);
					Instruction::CreateUser(
						try!(parse_type(&name.0)),
						try!(parse_type(&server.0)),
						user)
				},
				_ => {
					Instruction::CreateServer(try!(parse_type(&chunk)))
				}
			}
		} else if let Symbol::If = chunk[0].symbol {
			try!(parse_ifblock(&chunk))
		} else {
			// expressions
			try!(parse_expression(&chunk, SyntaxErrorFactory::from_symbols(&chunk)))
		};
		ret.push(inst);
	}
	Ok(ret)
}
