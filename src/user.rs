use interpreter::Interpreter;
use mail::Mail;
use instruction::Instruction;
use std::rc::Rc;
use std::fmt;
use types::Type;
use environment::Environment;
use regex;

/// A Tuple that represents a username + servername combo.
#[derive(Clone, Debug)]
pub struct UserPath(pub String, pub String);

impl UserPath {
    /// Returns a reference to this UserPath's username.
    fn get_username(&self) -> &str {
        &self.0
    }
    /// Returns a reference to this UserPath's server.
    fn get_servername(&self) -> &str {
        &self.1
    }
}

#[derive(Clone)]
pub struct User {
	pub func: UserType,
	pub env: Environment
}

impl fmt::Debug for User {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "User {{name: {}}}", self.env.path.get_username())
	}
}

impl User {
	pub fn create_user_external(name: &str, func: Box<UserExtFunc>) -> User {
		User {
			func: UserType::External(Rc::new(func)),
			env: Environment::new(name, "")
		}
	}

	pub fn create_user_internal(name: &str, instructions: Vec<(String, Vec<Instruction>)>)
	-> User {
		User {
			func: UserType::Internal(instructions.iter().map(
				|v|(regex::Regex::new(&v.0).unwrap(), v.1.clone())).collect()),
			env: Environment::new(name, "")
		}
	}

	pub fn create_mail(&self, to: UserPath, subject: &str, message: &str) -> Mail {
		Mail {
			from: self.env.path.clone(),
			to: to,
			subject: subject.to_string(),
			message: message.to_string(),
			attachments: Vec::new()
		}
	}

	pub fn send(&mut self, mut inter: &mut Interpreter, mail: &Mail) {
		self.env.path = mail.to.clone();
		self.env.set("subject", Type::Text(mail.subject.clone()));
		self.env.set("content", Type::Text(mail.message.clone()));
		self.env.set("sender", Type::UserPath(mail.from.clone()));
		self.env.set("self", Type::UserPath(mail.to.clone()));
		self.env.set("attachments", Type::Tuple(mail.attachments
			.iter()
			.map(|v|Type::Text(v.clone()))
			.collect()
		));
		match self.func {
			UserType::External(ref b) => {
				(**b)(self, &mut inter, &mail);
			},
			UserType::Internal(ref v) => {
				for matcher in v {
					if matcher.0.is_match(&mail.subject) {
						inter.run(&matcher.1, &mut self.env);
						break;
					}
				}
			}
		}
	}
}

pub type UserExtFunc = Fn(&User, &mut Interpreter, &Mail);

#[derive(Clone)]
pub enum UserType {
	External(Rc<Box<UserExtFunc>>),
	Internal(Vec<(regex::Regex, Vec<Instruction>)>)
}
