use user::UserPath;
use types::Type;
use error;
use parser::Parser;

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

impl Symbol {
	pub fn get_type(&self, parser: &Parser) -> Option<Type> {
		match *self {
			Symbol::Text(ref val) => Some(Type::Text(val.clone())),
			Symbol::UserPath(ref val) => Some(Type::UserPath(val.clone())),
			Symbol::Parenthesis(ref val) => {
				// let mut is_comma = false;
				// Some(Type::Tuple(val.0.iter().filter(
				// 	|_| {is_comma = !is_comma;is_comma}
				// ).map(
				// 	|v| v.symbol.get_type().unwrap()
				// ).collect()))
				if val.is_comma_delimited() {
					Some(Type::Tuple(val.split_commas()
						.iter().map(
							|v| if v.len() == 1 {
								v[0].symbol.get_type(&parser).unwrap()
							} else {
								Type::Expression(Box::new(parser.parse_expression(&v).unwrap()))
							}
						).collect()
					))
				} else {
					Some(Type::Expression(Box::new(parser.parse_expression(&val.0).unwrap())))
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
			_ => (false, 0)
		}
	}
}
