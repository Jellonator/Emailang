// use server;
use user::*;
use mail::*;
use interpreter::Interpreter;
use types::Type;
use environment::Environment;

#[derive(Debug, Clone)]
pub struct CondBlock {
	pub cond: Option<Type>,
	pub block: Vec<Instruction>,
	pub elseblock: Option<Box<CondBlock>>
}

impl CondBlock {
	pub fn append_block(&mut self, condblock: CondBlock) {
		if let None = self.elseblock {
			self.elseblock = Some(Box::new(condblock));
		} else {
			self.elseblock.as_mut().unwrap().append_block(condblock);
		}
	}

	pub fn call(&self, inter: &mut Interpreter, from: &UserPath, env: &mut Environment) {
		let do_thing = match self.cond {
			None => true,
			Some(ref t) => t.get_bool(inter, from, env)
		};
		if do_thing {
			inter.run(&self.block, from, env);
		} else {
			match self.elseblock {
				Some(ref eb) => eb.call(inter, from, env),
				None => {}
			}
		}
	}
}

#[derive(Clone, Debug)]
pub enum Instruction {
	CreateServer(Type),
	CreateUser(Type, Type, UserDef),
	MailTo(Type, Type),
	Concatenate(Type, Type),
	GetEnv(Type),
	Slice(Type, Option<Type>, Option<Type>),
	Index(Type, Type),
	Assign(Type, Type),
	IfBlock(CondBlock),
	Modify(Type, Type)
}

impl Instruction {
	pub fn call(&self, inter: &mut Interpreter, from: &UserPath, env: &mut Environment) -> Type {
		match *self {
			Instruction::CreateServer(ref name) => {
				let servername = name.get_string(inter, from, env).unwrap();
				inter.add_server(&servername);
			},
			Instruction::CreateUser(ref name, ref server, ref userdef) => {
				let username = name.get_string(inter, from, env).unwrap();
				let servername = server.get_string(inter, from, env).unwrap();
				inter.add_user(&username, &servername, userdef);
			},
			Instruction::MailTo(ref draft, ref name) => {
				let d = draft.get_draft(inter, from, env).unwrap();
				let target = name.get_user(inter, from, env).unwrap();
				inter.mail(Mail {
					subject: d.subject,
					message: d.message,
					attachments: d.attachments.clone(),
					to: target,
					from: from.clone()
				});
				return draft.clone();
			},
			Instruction::Concatenate(ref lval, ref rval) => {
				let lstr = lval.get_string(inter, from, env);
				let rstr = rval.get_string(inter, from, env);
				match (lstr, rstr) {
					(Some(ref lstringval), Some(ref rstringval)) => {
						return Type::Text(lstringval.clone() + rstringval);
					},
					(None, Some(_)) => {
						let mut tleft = lval.get_tuple(inter, from, env).unwrap();
						tleft.push(rval.clone());
						return Type::Tuple(tleft);
					},
					(Some(_), None) => {
						let mut tright = rval.get_tuple(inter, from, env).unwrap();
						let mut tleft:Vec<Type> = Vec::new();
						tleft.push(lval.clone());
						tleft.append(&mut tright);
						return Type::Tuple(tleft);
					},
					(None, None) => {
						let mut tleft = lval.get_tuple(inter, from, env).unwrap();
						let mut tright = rval.get_tuple(inter, from, env).unwrap();
						tleft.append(&mut tright);
						return Type::Tuple(tleft);
					},
				}
			},
			Instruction::GetEnv(ref val) => {
				let rawkey = val.get_string(inter, from, env);
				let rawtuple = val.get_tuple(inter, from, env);
				return if let Some(tuple) = rawtuple {
					Type::Tuple(tuple.iter().map(
						|v|{let s = &v.get_string(inter, from, env).unwrap();env.get(s)}
						).collect())
				} else if let Some(key) = rawkey {
					env.get(&key)
				} else {
					panic!();
				};
			},
			Instruction::Index(ref val, ref pos) => {
				return val.index(pos.get_num(inter, from, env).unwrap(), inter, from, env).unwrap();
			},
			Instruction::Slice(ref val, ref a, ref b) => {
				let start = match *a {
					Some(ref val) => val.get_num(inter, from, env).unwrap(),
					None => 0
				};
				let end = match *b {
					Some(ref val) => val.get_num(inter, from, env).unwrap(),
					None => val.len(inter, from, env).unwrap() as isize
				};
				return val.slice(start, end, inter, from, env).unwrap();
			},
			Instruction::Assign(ref to, ref val) => {
				match to.get_string(inter, from, env) {
					Some(ref s) => {
						let content = val.resolve(inter, from, env);
						env.set(s, content);
					},
					None => {
						match (to.get_tuple(inter, from, env), val.get_tuple(inter, from, env)) {
							(Some(ref tuple), Some(ref res)) => {
								for i in 0..tuple.len() {
									let s = &tuple[i].get_string(inter, from, env).unwrap();
									let content = res[i].resolve(inter, from, env);
									env.set(s, content);
								}
							},
							_ => panic!()
						}
					}
				}
				return val.clone();
			},
			Instruction::IfBlock(ref b) => {
				b.call(inter, from, env);
			},
			Instruction::Modify(ref val, ref modifier) => {
				return modifier.modify(val, inter, from, env).unwrap();
			}
		}
		Type::Null
	}
}
