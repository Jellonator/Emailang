use interpreter::Interpreter;
use mail::Mail;
use instruction::Instruction;
use std::rc::Rc;
use std::fmt;
use types::Type;
use environment::Environment;
use regex;

/// A Tuple that represents a username + servername combo.
#[derive(Clone)]
pub struct UserPath(pub String, pub String);

impl fmt::Debug for UserPath {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}@{}", &self.0, &self.1)
	}
}

impl UserPath {
	/// Returns a reference to this UserPath's username.
	pub fn get_username(&self) -> &str {
		&self.0
	}
	/// Returns a reference to this UserPath's server.
	pub fn get_servername(&self) -> &str {
		&self.1
	}
	/// Creates a new anonymous UserPath
	pub fn new_anon() -> UserPath {
		UserPath (
			"Anonymous".to_string(),
			"anon".to_string()
		)
	}
}

#[derive(Clone)]
pub struct UserDef {
	pub func: Rc<UserType>
}

impl fmt::Debug for UserDef {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "UserDef")
	}
}

impl UserDef {
	pub fn create_def_external(func: Box<UserExtFunc>) -> UserDef {
		UserDef {
			func: Rc::new(UserType::External(func))
		}
	}

	pub fn create_def_internal(instructions: Vec<(String, Vec<Instruction>)>)
	-> UserDef {
		UserDef {
			func: Rc::new(UserType::Internal(instructions.iter().map(
				|v|(regex::Regex::new(&v.0).unwrap(), v.1.clone())).collect()))
		}
	}

	pub fn create_user(&self) -> User {
		User {
			func: self.func.clone(),
			env: Environment::new()
		}
	}
}

#[derive(Clone)]
pub struct User {
	pub func: Rc<UserType>,
	pub env: Environment
}

impl fmt::Debug for User {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "UserDef")
	}
}

impl User {
	pub fn get_userdef(&self) -> UserDef {
		UserDef {
			func: self.func.clone()
		}
	}

	pub fn send(&mut self, mut inter: &mut Interpreter, mail: &Mail) {
		self.env.set("subject", Type::Text(mail.subject.clone()));
		self.env.set("content", Type::Text(mail.message.clone()));
		self.env.set("sender", Type::UserPath(mail.from.clone()));
		self.env.set("self", Type::UserPath(mail.to.clone()));
		self.env.set("attachments", Type::Tuple(mail.attachments
			.iter()
			.map(|v|Type::Text(v.clone()))
			.collect()
		));
		match *self.func {
			UserType::External(ref b) => {
				(**b)(self, &mut inter, &mail);
			},
			UserType::Internal(ref v) => {
				for matcher in v {
					if matcher.0.is_match(&mail.subject) {
						inter.run(&matcher.1, &mail.to, &mut self.env);
						break;
					}
				}
			}
		}
	}
}

pub type UserExtFunc = Fn(&User, &mut Interpreter, &Mail);

pub enum UserType {
	External(Box<UserExtFunc>),
	Internal(Vec<(regex::Regex, Vec<Instruction>)>)
}
