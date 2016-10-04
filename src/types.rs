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
	pub fn get_bool(&self, inter: &mut Interpreter, env: &mut Environment) -> bool {
		match *self {
			Type::Null => false,
			Type::Text(ref s) => {
				!["false", "0", ""].contains(&s.to_lowercase().as_str())
			},
			Type::Expression(_) => self.resolve(inter, env).get_bool(inter, env),
			_ => true
		}
	}

	pub fn resolve(&self, inter: &mut Interpreter, env: &mut Environment) -> Type {
		match *self {
			Type::Expression(ref exp) => {
				exp.call(inter, env).resolve(inter, env)
			},
			Type::Tuple(ref tuple) => {
				Type::Tuple(tuple.iter().map(|v|v.resolve(inter, env)).collect())
			},
			ref other => other.clone()
		}
	}

	pub fn len(&self, inter: &mut Interpreter, env: &mut Environment) -> Option<usize> {
		match *self {
			Type::Tuple(ref vec) => Some(vec.len()),
			Type::Text(ref text) => Some(text.chars().count()),
			Type::Expression(_) => self.resolve(inter, env).len(inter, env),
			_ => None
		}
	}

	pub fn index(&self, pos: isize, inter: &mut Interpreter, env: &mut Environment) -> Option<Type> {
		let selflen = self.len(inter, env).unwrap();
		let pos = if pos < 0 {
			((selflen as isize) + pos) as usize
		} else {
			pos as usize
		};
		match *self {
			Type::Tuple(ref vec) => Some(vec[pos].clone()),
			Type::Text(ref text) => Some(Type::Text(text.chars().nth(pos).unwrap().to_string())),
			Type::Expression(_) => self.resolve(inter, env).index(pos as isize, inter, env),
			_ => None
		}
	}

	pub fn slice(&self, a: isize, b: isize, inter: &mut Interpreter, env: &mut Environment) -> Option<Type> {
		let selflen = self.len(inter, env).unwrap();
		let a = if a < 0 {
			((selflen as isize) + a) as usize
		} else {
			a as usize
		};
		let b = if b < 0 {
			((selflen as isize) + b) as usize
		} else {
			b as usize
		};
		match *self {
			Type::Tuple(ref vec) => Some(Type::Tuple(vec[a..b].to_vec())),
			Type::Text(ref text) => {
				let chars = text.chars();
				Some(Type::Text(chars.skip(a as usize).take(b-a).collect()))
			},
			Type::Expression(_) => self.resolve(inter, env)
				.slice(a as isize, b as isize, inter, env),
			_ => None
		}
	}

	pub fn is_null(&self) -> bool {
		if let Type::Null = *self {
			true
		} else {
			false
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
			Type::Expression(_) => self.resolve(inter, env).get_string(inter, env),
			_ => None
		}
	}

	pub fn get_tuple(&self, inter: &mut Interpreter, env: &mut Environment) -> Option<Vec<Type>> {
		match *self {
			Type::Tuple(ref v) => Some(v.clone()),
			Type::Expression(_) => self.resolve(inter, env).get_tuple(inter, env),
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
			Type::Expression(_) => self.resolve(inter, env).get_draft(inter, env),
			_ => None
		}
	}

	pub fn get_user(&self, inter: &mut Interpreter, env: &mut Environment) -> Option<UserPath> {
		match *self {
			Type::UserPath(ref val) => Some(val.clone()),
			Type::Expression(_) => self.resolve(inter, env).get_user(inter, env),
			_ => None
		}
	}
}
