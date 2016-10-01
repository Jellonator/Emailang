use interpreter::Interpreter;
use mail::Mail;
use instruction::Instruction;
use std::collections::HashMap;
use std::rc::Rc;
use std::fmt;
use types::Type;
use environment::Environment;
use server::Server;

#[derive(Clone, Debug)]
pub struct UserPath(pub String, pub String);

#[derive(Clone)]
pub struct User {
	pub name: String,
	func: UserType
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
			func: UserType::External(Rc::new(func))
		}
	}

	pub fn create_user_internal(name: &str, instructions: HashMap<String, Vec<Instruction>>)
	-> User {
		User {
			name: name.to_string(),
			func: UserType::Internal(instructions)
		}
	}

	pub fn send(&mut self, mut inter: &mut Interpreter, mail: &Mail, server: &Server) {
		// println!("Received mail!");
		match self.func {
			UserType::External(ref mut b) => {
				(**b)(&mut inter, &mail);
			},
			UserType::Internal(ref v) => {
				let mut env = Environment::new(&self.name, &server.name);
				env.set("subject", Type::Text(mail.subject.clone()));
				env.set("content", Type::Text(mail.message.clone()));
				for i in 0..mail.attachments.len() {
					env.set(&("attach".to_string() + &i.to_string()),
						Type::Text(mail.attachments[i].clone()));
				}
				env.set("attachments", Type::Tuple(mail.attachments
					.iter()
					.map(|v|Type::Text(v.clone()))
					.collect()
				));
				match v.get(&mail.subject) {
					Some(ref ivec) => {
						inter.run(&ivec, &mut env);
					},
					None => {}
				}
			}
		}
	}
}

pub type UserExtFunc = Fn(&mut Interpreter, &Mail);

#[derive(Clone)]
pub enum UserType {
	External(Rc<Box<UserExtFunc>>),
	Internal(HashMap<String, Vec<Instruction>>)
}
