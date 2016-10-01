use user::UserPath;
use types::Type;
use error;
use parser;
use parser::Parser;
use std::fmt;

#[derive(Clone)]
pub struct Block(pub Vec<SymbolDef>);

impl Block {
	pub fn is_comma_delimited(&self) -> bool {
		for val in &self.0 {
			match val.symbol {
				Symbol::Comma => return true,
				_ => {}
			}
		}
		return false;
	}

	pub fn split_commas(&self) -> Vec<Vec<SymbolDef>> {
		let mut ret = vec![];
		let mut vec = vec![];
		for val in &self.0 {
			match val.symbol {
				Symbol::Comma => {
					ret.push(vec);
					vec = vec![];
				},
				_ => {
					vec.push(val.clone())
				}
			}
		}
		ret.push(vec);
		ret
	}
}

/* Symbols:
 * !        - define
 * <a@b>    - user
 * "a"      - text
 * a        - identifier
 * {a;b;c;} - curlybraced
 * (a,b,c)  - parenthesis
 * >        - arrow
 * ,        - comma
 * ;        - semicolon
**/

#[derive(Clone)]
pub enum Symbol {
	// Structures
	CurlyBraced(Block),
	Parenthesis(Block),
	// Types
	UserPath(UserPath),
	Identifier(String),
	Text(String),
	// Syntax
	Comma,
	Semicolon,
	Receive(Box<SymbolDef>),
	// Operators
	Define,
	Arrow,
	Addition,
}

#[derive(Clone)]
pub struct SymbolDef {
	pub errfactory: error::ErrorFactory,
	pub symbol: Symbol
}

impl fmt::Debug for SymbolDef {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", match self.symbol {
			Symbol::CurlyBraced(_) => "CurlyBraced",
			Symbol::Parenthesis(_) => "Parenthesis",
			Symbol::UserPath(_) => "Userpath",
			Symbol::Identifier(_) => "Identifier",
			Symbol::Text(_) => "Text",
			Symbol::Comma => "Comma",
			Symbol::Semicolon => "Semicolon",
			Symbol::Receive(_) => "Receiver",
			Symbol::Define => "Define",
			Symbol::Arrow => "Arrow",
			Symbol::Addition => "Addition",
		})
	}
}

impl Symbol {
	pub fn get_type(&self, parser: &Parser) -> Option<Type> {
		match *self {
			Symbol::Text(ref val) => Some(Type::Text(val.clone())),
			Symbol::Identifier(ref val) => Some(Type::Text(val.clone())),
			Symbol::UserPath(ref val) => Some(Type::UserPath(val.clone())),
			Symbol::Parenthesis(ref val) => {
				if val.is_comma_delimited() {
					let mut tuple = Vec::new();
					for v in val.split_commas() {
						if parser::is_expression(&v) {
							tuple.push(Type::Expression(Box::new(parser.parse_expression(&v).unwrap())));
						} else {
							for symdef in v {
								tuple.push(symdef.symbol.get_type(&parser).unwrap());
							}
						}
					}
					Some(Type::Tuple(tuple))
				} else {
					if parser::is_expression(&val.0) {
						Some(Type::Expression(Box::new(parser.parse_expression(&val.0).unwrap())))
					} else {
						assert!(val.0.len() == 1);
						Some(val.0[0].symbol.get_type(&parser).unwrap())
					}
				}
			},
			_ => None
		}
	}

	pub fn get_operator(&self) -> (bool, usize) {
		match *self {
			Symbol::Addition => (true, 1000),
			Symbol::Arrow => (true, 1001),
			Symbol::Comma => (true, 2000),
			Symbol::Receive(_) => (true, 1),
			_ => (false, 0)
		}
	}
}
