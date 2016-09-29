use user::UserPath;
use types::Type;

#[derive(Debug)]
pub struct Block(pub Vec<Symbol>);

/* Symbols:
 * !        - define
 * <a@b>    - user
 * "a"      - text
 * a        - name
 * {a;b;c;} - curlybraced
 * (a,b,c)  - parenthesis
**/

#[derive(Debug)]
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

impl Symbol {
	pub fn get_type(&self) -> Option<Type> {
		match *self {
			Symbol::Text(ref val) => Some(Type::Text(val.clone())),
			Symbol::UserPath(ref val) => Some(Type::UserPath(val.clone())),
			Symbol::Parenthesis(ref val) => {
				Some(Type::Tuple(val.0.iter().map(|v|v.get_type().unwrap()).collect()))
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
