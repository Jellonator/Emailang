// use server;
use user::*;
use mail::*;
use interpreter::Interpreter;
use types::Type;

#[derive(Clone)]
pub enum Instruction {
	CreateServer(String),
	CreateUser(String, User),
	MailTo(Draft, UserPath)
}


impl Instruction {
	pub fn call(&self, inter: &mut Interpreter) -> Type{
		match *self {
			Instruction::CreateServer(ref name) => {
				inter.add_server(name);
			},
			Instruction::CreateUser(ref name, ref user) => {
				inter.add_user(name, user);
			},
			Instruction::MailTo(ref draft, ref name) => {
				inter.mail(&Mail {
					subject: draft.subject.clone(),
					message: draft.message.clone(),
					to: name.clone()
				});
			}
		}
		Type::Null
	}
}
