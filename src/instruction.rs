// use server;
use user::*;
use mail::*;
use interpreter::Interpreter;
use types::Type;
use environment::Environment;

#[derive(Clone, Debug)]
pub enum Instruction {
	CreateServer(String),
	CreateUser(String, User),
	MailTo(Type, Type),
	Concatenate(Type, Type),
	GetEnv(Type),
	Slice(Type, Option<isize>, Option<isize>),
	Index(Type, isize),
	Assign(Type, Type)
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
						|v|{let s = &v.get_string(inter, env).unwrap();env.get(s)}
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
					None => val.len(inter, env).unwrap() as isize
				};
				return val.slice(start, end, inter, env).unwrap();
			},
			Instruction::Assign(ref to, ref val) => {
				match to.get_string(inter, env) {
					Some(ref s) => {
						let content = val.resolve(inter, env);
						env.set(s, content);
						return val.clone();
					},
					None => {
						match (to.get_tuple(inter, env), val.get_tuple(inter, env)) {
							(Some(ref tuple), Some(ref res)) => {
								for i in 0..tuple.len() {
									let s = &tuple[i].get_string(inter, env).unwrap();
									let content = res[i].resolve(inter, env);
									env.set(s, content);
								}
							},
							_ => panic!()
						}
					}
				}
			}
		}
		Type::Null
	}
}
