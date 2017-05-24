use user::UserPath;
use instruction::Instruction;
use interpreter::Interpreter;
use mail::Draft;
use environment::Environment;
use std::str::FromStr;
use modifier::ModifierFunc;

#[derive(Clone, Debug)]
pub enum Type {
	Null,
	Text(String),
	UserPath(Box<Type>, Box<Type>),
	Tuple(Vec<Type>),
	Expression(Box<Instruction>)
}

impl Type {
	pub fn get_num<T>(&self, inter: &mut Interpreter, from: &UserPath,
	                  env: &mut Environment) -> Option<T>
	where T: FromStr {
		match *self {
			Type::Text(ref s) => s.parse::<T>().ok(),
			Type::Expression(_) => self.resolve(inter, from, env).get_num(inter, from, env),
			_ => None
		}
	}

	fn get_modname(&self, inter: &mut Interpreter, from: &UserPath,
	               env: &mut Environment) -> String {
		match *self {
			Type::Text(ref s) => s.clone(),
			Type::Tuple(ref t) => t[0].get_string(inter, from, env).unwrap(),
			Type::Expression(_) => self.resolve(inter, from, env).get_modname(inter, from, env),
			_ => panic!()
		}
	}

	fn get_modargs(&self, inter: &mut Interpreter, from: &UserPath,
	               env: &mut Environment) -> Vec<Type> {
		match *self {
			Type::Text(_) => Vec::new(),
			Type::Tuple(ref t) => t[1..].to_vec(),
			Type::Expression(_) => self.resolve(inter, from, env).get_modargs(inter, from, env),
			_ => panic!()
		}
	}

	pub fn modify(&self, other: &Type, inter: &mut Interpreter, from: &UserPath,
	              env: &mut Environment) -> Option<Type> {
		let mod_name = self.get_modname(inter, from, env);
		let mod_args = self.get_modargs(inter, from, env);
		// yet another hack here :(
		let ptr;
		match inter.modifiers.get(&mod_name) {
			Some(func) => {
				use std::borrow::Borrow;
				ptr = func.borrow() as *const ModifierFunc;
			},
			None => {return None;}
		}
		unsafe {
			(*ptr)(other, inter, from, env, &mod_args)
		}
	}

	pub fn get_bool(&self, inter: &mut Interpreter, from: &UserPath,
	                env: &mut Environment) -> bool {
		match *self {
			Type::Null => false,
			Type::Text(ref s) => {
				!["false", "0", ""].contains(&s.to_lowercase().as_str())
			},
			Type::Tuple(ref t) => t.len() > 0,
			Type::Expression(_) => self.resolve(inter, from, env).get_bool(inter, from, env),
			_ => true
		}
	}

	pub fn resolve(&self, inter: &mut Interpreter, from: &UserPath, env: &mut Environment) -> Type {
		match *self {
			Type::Expression(ref exp) => {
				exp.call(inter, from, env).resolve(inter, from, env)
			},
			Type::Tuple(ref tuple) => {
				Type::Tuple(tuple.iter().map(|v|v.resolve(inter, from, env)).collect())
			},
			ref other => other.clone()
		}
	}

	pub fn len(&self, inter: &mut Interpreter, from: &UserPath,
	           env: &mut Environment) -> Option<usize> {
		match *self {
			Type::Tuple(ref vec) => Some(vec.len()),
			Type::Text(ref text) => Some(text.chars().count()),
			Type::Expression(_) => self.resolve(inter, from, env).len(inter, from, env),
			_ => None
		}
	}

	pub fn index(&self, pos: isize, inter: &mut Interpreter, from: &UserPath,
	             env: &mut Environment) -> Option<Type> {
		let selflen = self.len(inter, from, env).unwrap();
		let pos = if pos < 0 {
			((selflen as isize) + pos) as usize
		} else {
			pos as usize
		};
		match *self {
			Type::Tuple(ref vec) => Some(vec[pos].clone()),
			Type::Text(ref text) => Some(Type::Text(text.chars().nth(pos).unwrap().to_string())),
			Type::Expression(_) => self.resolve(inter, from, env)
			                           .index(pos as isize, inter, from, env),
			_ => None
		}
	}

	pub fn slice(&self, a: isize, b: isize, inter: &mut Interpreter, from: &UserPath,
	             env: &mut Environment) -> Option<Type> {
		let selflen = self.len(inter, from, env).unwrap();
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
				Some(Type::Text(chars.skip(a).take(b-a).collect()))
			},
			Type::Expression(_) => self.resolve(inter, from, env)
				.slice(a as isize, b as isize, inter, from, env),
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
			Type::UserPath(_, _) => "user",
			Type::Expression(_) => "expression"
		}
	}

	pub fn get_string(&self, inter: &mut Interpreter, from: &UserPath,
	                  env: &mut Environment) -> Option<String> {
		match *self {
			Type::Text(ref val) => Some(val.clone()),
			Type::Expression(_) => self.resolve(inter, from, env).get_string(inter, from, env),
			Type::UserPath(ref name, ref server) => Some(
				format!("{}@{}", name.get_string(inter, from, env).unwrap(),
				                 server.get_string(inter, from, env).unwrap())),
			_ => None
		}
	}

	pub fn get_tuple(&self, inter: &mut Interpreter, from: &UserPath,
	                 env: &mut Environment) -> Option<Vec<Type>> {
		match *self {
			Type::Tuple(ref v) => Some(v.clone()),
			Type::Expression(_) => self.resolve(inter, from, env).get_tuple(inter, from, env),
			_ => None
		}
	}

	pub fn unpack(&self, inter: &mut Interpreter, from: &UserPath,
	              env: &mut Environment) -> Vec<Type> {
		match self.get_tuple(inter, from, env) {
			Some(v) => v,
			None => vec![self.clone()]
		}
	}

	pub fn get_draft(&self, inter: &mut Interpreter, from: &UserPath,
	                 env: &mut Environment) -> Option<Draft> {
		match *self {
			Type::Tuple(ref t) => {
				Some(Draft {
					subject: t.get(0).map(
						|v|v.get_string(inter, from, env).unwrap_or("".to_string())
					).unwrap_or("".to_string()),
					message: t.get(1).map(
						|v|v.get_string(inter, from, env).unwrap_or("".to_string())
					).unwrap_or("".to_string()),
					attachments: (2..).take_while(|v|*v<t.len()).map(
						|v|t[v].get_string(inter, from, env).unwrap_or("".to_string())
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
			Type::Expression(_) => self.resolve(inter, from, env).get_draft(inter, from, env),
			_ => None
		}
	}

	pub fn get_user(&self, inter: &mut Interpreter, from: &UserPath,
	                env: &mut Environment) -> Option<UserPath> {
		match *self {
			Type::UserPath(ref name, ref server) => {
				let a = name.get_string(inter, from, env);
				let b = server.get_string(inter, from, env);
				match (a, b) {
					(Some(a), Some(b)) => {
						Some(UserPath(a, b))
					},
					_ => None
				}
			},
			Type::Expression(_) => self.resolve(inter, from, env).get_user(inter, from, env),
			_ => None
		}
	}
}
