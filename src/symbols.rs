use user::UserPath;
use types::Type;
use error;

#[derive(Clone)]
pub struct Block(pub Vec<SymbolDef>);

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
	Define,
	CurlyBraced(Block),
	Parenthesis(Block),
	UserPath(UserPath),
	Identifier(String),
	Text(String),
	// Draft(Block),
	Comma,
	Arrow,
	Semicolon
}

#[derive(Clone)]
pub struct SymbolDef {
	pub errfactory: error::ErrorFactory,
	pub symbol: Symbol
}

impl Symbol {
	pub fn get_type(&self) -> Option<Type> {
		match *self {
			Symbol::Text(ref val) => Some(Type::Text(val.clone())),
			Symbol::UserPath(ref val) => Some(Type::UserPath(val.clone())),
			Symbol::Parenthesis(ref val) => {

				// println!("Typeof {:?}", val.0);
				let mut is_comma = false;
				Some(Type::Tuple(val.0.iter().filter(
					|_| {is_comma = !is_comma;is_comma}
				).map(
					|v| v.symbol.get_type().unwrap()
				).collect()))
			},
			_ => None
		}
	}

	pub fn get_operator(&self) -> (bool, usize) {
		match *self {
			Symbol::Comma => (true, 1001),
			Symbol::Arrow => (true, 1000),
			_ => (false, 0)
		}
	}
}
