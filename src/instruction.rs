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
	Slice(Type, Option<usize>, Option<usize>),
	Index(Type, usize),
	Assign(Type, Type)
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
			},
			Instruction::Slice(_, a, b) => {
				write!(f, "Slice variable from {:?} to {:?}", a, b)
			},
			Instruction::Index(_, pos) => {
				write!(f, "Index variable at {}", pos)
			},
			Instruction::Assign(_, _) => {
				write!(f, "Assign a variable")
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
					from: UserPath(env.username.clone(), env.server.clone())
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
				let rawkey = val.get_string(inter, env);
				let rawtuple = val.get_tuple(inter, env);
				return if let Some(tuple) = rawtuple {
					Type::Tuple(tuple.iter().map(
						|v|{let s = &v.get_string(&mut inter, &mut env).unwrap();env.get(s)}
						).collect())
				} else if let Some(key) = rawkey {
					env.get(&key)
				} else {
					panic!();
				};
			},
			Instruction::Index(ref val, pos) => {
				return val.index(pos, inter, env).unwrap();
			},
			Instruction::Slice(ref val, a, b) => {
				let start = match a {
					Some(val) => val,
					None => 0
				};
				let end = match b {
					Some(val) => val,
					None => val.len(inter, env).unwrap()
				};
				return val.slice(start, end, inter, env).unwrap();
			},
			Instruction::Assign(ref to, ref val) => {
				let s = &to.get_string(inter, env).unwrap();
				let content = val.resolve(inter, env);
				env.set(s, content);
				return val.clone();
			}
		}
		Type::Null
	}
}
