use user::UserPath;
use instruction::Instruction;
use interpreter::Interpreter;
use mail::Draft;
use environment::Environment;

#[derive(Clone, Debug)]
pub enum Type {
	Null,
	Text(String),
	UserPath(UserPath),
	Tuple(Vec<Type>),
	Expression(Box<Instruction>)
}

impl Type {
	pub fn resolve(&self, inter: &mut Interpreter, env: &mut Environment) -> Type {
		match *self {
			Type::Expression(ref exp) => {
				exp.call(inter, env)
			},
			ref other => other.clone()
		}
	}

	pub fn get_typename(&self) -> &'static str {
		match *self {
			Type::Null => "null",
			Type::Text(_) => "text",
			Type::Tuple(_) => "tuple",
			Type::UserPath(_) => "user",
			Type::Expression(_) => "expression"
		}
	}

	pub fn get_string(&self, inter: &mut Interpreter, env: &mut Environment) -> Option<String> {
		match *self {
			Type::Text(ref val) => Some(val.clone()),
			Type::Expression(_) => self.resolve(inter, env)
				.get_string(inter, env),
			_ => None
		}
	}

	pub fn get_tuple(&self, inter: &mut Interpreter, env: &mut Environment) -> Option<Vec<Type>> {
		match *self {
			Type::Tuple(ref v) => Some(v.clone()),
			Type::Expression(_) => self.resolve(inter, env)
				.get_tuple(inter, env),
			_ => None
		}
	}

	pub fn get_draft(&self, inter: &mut Interpreter, env: &mut Environment) -> Option<Draft> {
		match *self {
			Type::Tuple(ref t) => {
				Some(Draft {
					subject: t.get(0).map(
						|v|v.get_string(inter, env).unwrap_or("".to_string())
					).unwrap_or("".to_string()),
					message: t.get(1).map(
						|v|v.get_string(inter, env).unwrap_or("".to_string())
					).unwrap_or("".to_string()),
					attachments: (2..).take_while(|v|*v<t.len()).map(
						|v|t[v].get_string(inter, env).unwrap_or("".to_string())
					).collect()
				})
			},
			Type::Text(ref val) => {
				Some(Draft {
					subject: val.to_string(),
					message: "".to_string(),
					attachments: Vec::new()
				})
			},
			Type::Expression(_) => self.resolve(inter, env)
				.get_draft(inter, env),
			_ => None
		}
	}

	pub fn get_user(&self, inter: &mut Interpreter, env: &mut Environment) -> Option<UserPath> {
		match *self {
			Type::UserPath(ref val) => Some(val.clone()),
			Type::Expression(_) => self.resolve(inter, env)
				.get_user(inter, env),
			_ => None
		}
	}
}
