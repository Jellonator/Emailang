use user::UserPath;

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
	Name(String),
	Identifier(String),
	Text(String),
	// Draft(Block),
	Comma,
	Arrow,
	Semicolon
}

impl Symbol {
	pub fn get_operator(&self) -> (bool, usize) {
		match *self {
			Symbol::Comma => (true, 1001),
			Symbol::Arrow => (true, 1000),
			_ => (false, 0)
		}
	}
}
