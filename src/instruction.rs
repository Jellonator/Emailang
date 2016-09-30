// use server;
use user::*;
use mail::*;
use interpreter::Interpreter;
use types::Type;
use std::fmt;
use environment::Environment;

#[derive(Clone, Debug)]
pub enum Instruction {
	CreateServer(String),
	CreateUser(String, User),
	MailTo(Type, Type),
	Concatenate(Type, Type),
	GetEnv(Type),
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
			Instruction::Concatenate(_, _) => {
				write!(f, "Concatenate two strings")
			},
			Instruction::GetEnv(_) => {
				write!(f, "Get variable from environment")
			}
		}
	}
}

impl Instruction {
	pub fn call(&self, mut inter: &mut Interpreter, mut env: &mut Environment) -> Type {
		match *self {
			Instruction::CreateServer(ref name) => {
				inter.add_server(name);
			},
			Instruction::CreateUser(ref name, ref user) => {
				inter.add_user(name, user);
			},
			Instruction::MailTo(ref draft, ref name) => {
				let d = draft.get_draft(&mut inter, &mut env).unwrap();
				let target = name.get_user(&mut inter, &mut env).unwrap();

				inter.mail(&Mail {
					subject: d.subject,
					message: d.message,
					attachments: d.attachments.clone(),
					to: target,
				});
				return draft.clone();
			},
			Instruction::Concatenate(ref lval, ref rval) => {
				let lstr = lval.get_string(&mut inter, &mut env);
				let rstr = rval.get_string(&mut inter, &mut env);
				match (lstr, rstr) {
					(Some(ref lstringval), Some(ref rstringval)) => {
						return Type::Text(lstringval.clone() + rstringval);
					},
					(None, Some(_)) => {
						let mut tleft = lval.get_tuple(&mut inter, &mut env).unwrap();
						tleft.push(rval.clone());
						return Type::Tuple(tleft);
					},
					(Some(_), None) => {
						let mut tright = rval.get_tuple(&mut inter, &mut env).unwrap();
						let mut tleft:Vec<Type> = Vec::new();
						tleft.push(lval.clone());
						tleft.append(&mut tright);
						return Type::Tuple(tleft);
					},
					(None, None) => {
						let mut tleft = lval.get_tuple(&mut inter, &mut env).unwrap();
						let mut tright = rval.get_tuple(&mut inter, &mut env).unwrap();
						tleft.append(&mut tright);
						return Type::Tuple(tleft);
					},
				}
			},
			Instruction::GetEnv(ref val) => {
				let rawkey = val.get_string(&mut inter, &mut env);
				let rawtuple = val.get_tuple(&mut inter, &mut env);
				return if let Some(tuple) = rawtuple {
					Type::Tuple(tuple.iter().map(
						|v|{let s = &v.get_string(&mut inter, &mut env).unwrap();env.get(s)}
						).collect())
				} else if let Some(key) = rawkey {
					env.get(&key)
				} else {
					panic!();
				};
			}
		}
		Type::Null
	}
}
