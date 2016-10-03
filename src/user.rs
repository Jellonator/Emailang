use interpreter::Interpreter;
use mail::Mail;
use instruction::Instruction;
use std::rc::Rc;
use std::fmt;
use types::Type;
use environment::Environment;
use server::Server;
extern crate regex;

#[derive(Clone, Debug)]
pub struct UserPath(pub String, pub String);

#[derive(Clone)]
pub struct User {
	pub name: String,
	func: UserType,
	env: Environment
}

impl fmt::Debug for User {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "User {{name: {}}}", self.name)
	}
}

impl User {
	pub fn create_user_external(name: &str, func: Box<UserExtFunc>) -> User {
		User {
			name: name.to_string(),
			func: UserType::External(Rc::new(func)),
			env: Environment::new(name, "")
		}
	}

	pub fn create_user_internal(name: &str, instructions: Vec<(String, Vec<Instruction>)>)
	-> User {
		User {
			name: name.to_string(),
			func: UserType::Internal(instructions.iter().map(
				|v|(regex::Regex::new(&v.0).unwrap(), v.1.clone())).collect()),
			env: Environment::new(name, "")
		}
	}

	pub fn send(&mut self, mut inter: &mut Interpreter, mail: &Mail, server: &Server) {
		// println!("Received mail!");
		match self.func {
			UserType::External(ref mut b) => {
				(**b)(&mut inter, &mail);
			},
			UserType::Internal(ref v) => {
				// let mut env = Environment::new(&self.name, &server.name);
				// self.env.username = self.name
				self.env.server = server.name.clone();
				self.env.set("subject", Type::Text(mail.subject.clone()));
				self.env.set("content", Type::Text(mail.message.clone()));
				// for i in 0..mail.attachments.len() {
				// 	self.env.set(&("attach".to_string() + &i.to_string()),
				// 		Type::Text(mail.attachments[i].clone()));
				// }
				self.env.set("attachments", Type::Tuple(mail.attachments
					.iter()
					.map(|v|Type::Text(v.clone()))
					.collect()
				));
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

pub type UserExtFunc = Fn(&mut Interpreter, &Mail);

#[derive(Clone)]
pub enum UserType {
	External(Rc<Box<UserExtFunc>>),
	Internal(Vec<(regex::Regex, Vec<Instruction>)>)
}
