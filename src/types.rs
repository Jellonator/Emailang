use user::UserPath;
use instruction::Instruction;
use interpreter::Interpreter;
use mail::Draft;

#[derive(Clone, Debug)]
pub enum Type {
	Null,
	Text(String),
	UserPath(UserPath),
	Tuple(Vec<Type>),
	Expression(Box<Instruction>)
}

impl Type {
	pub fn resolve(&self, mut inter: &mut Interpreter) -> Type {
		match *self {
			Type::Expression(ref exp) => {
				exp.call(&mut inter)
			},
			ref other => other.clone()
		}
	}

	pub fn get_string(&self, mut inter: &mut Interpreter) -> Option<String> {
		match *self {
			Type::Text(ref val) => Some(val.clone()),
			Type::Expression(_) => self.resolve(&mut inter).get_string(&mut inter),
			_ => None
		}
	}

	pub fn get_tuple(&self, mut inter: &mut Interpreter) -> Option<Vec<Type>> {
		match *self {
			Type::Tuple(ref v) => Some(v.clone()),
			Type::Expression(_) => self.resolve(&mut inter).get_tuple(&mut inter),
			_ => None
		}
	}

	pub fn get_draft(&self, mut inter: &mut Interpreter) -> Option<Draft> {
		match *self {
			Type::Tuple(ref t) => {
				Some(Draft {
					subject: t.get(0).map(
						|v|v.get_string(&mut inter).unwrap_or("".to_string())
					).unwrap_or("".to_string()),
					message: t.get(1).map(
						|v|v.get_string(&mut inter).unwrap_or("".to_string())
					).unwrap_or("".to_string())
				})
			},
			Type::Text(ref val) => {
				Some(Draft {
					subject: val.to_string(),
					message: "".to_string()
				})
			},
			Type::Expression(_) => self.resolve(&mut inter).get_draft(&mut inter),
			_ => None
		}
	}

	pub fn get_user(&self, mut inter: &mut Interpreter) -> Option<UserPath> {
		match *self {
			Type::UserPath(ref val) => Some(val.clone()),
			Type::Expression(_) => self.resolve(&mut inter).get_user(&mut inter),
			_ => None
		}
	}
}
