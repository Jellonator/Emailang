// use server;
use user::*;
use mail::*;
use interpreter::Interpreter;
use types::Type;
use std::fmt;

#[derive(Clone, Debug)]
pub enum Instruction {
	CreateServer(String),
	CreateUser(String, User),
	MailTo(Type, Type)
}

impl fmt::Display for Instruction {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Instruction::CreateServer(ref name) => {
				write!(f, "Create server {}", name)
			},
			Instruction::CreateUser(ref name, _) => {
				write!(f, "Create user {}", name)
			},
			Instruction::MailTo(_, _) => {
				write!(f, "Send mail")
			},
		}
	}
}

impl Instruction {
	pub fn call(&self, mut inter: &mut Interpreter) -> Type {
		match *self {
			Instruction::CreateServer(ref name) => {
				inter.add_server(name);
			},
			Instruction::CreateUser(ref name, ref user) => {
				inter.add_user(name, user);
			},
			Instruction::MailTo(ref draft, ref name) => {
				// println!("{:?}", draft);
				let tuple = draft.get_tuple(&mut inter).unwrap();
				let target = name.get_user(&mut inter).unwrap();
				let subject = tuple[0].get_string(&mut inter).unwrap();
				let message = tuple[1].get_string(&mut inter).unwrap();
				inter.mail(&Mail {
					subject: subject,
					message: message,
					to: target
				});
				return draft.clone();
			}
		}
		Type::Null
	}
}
