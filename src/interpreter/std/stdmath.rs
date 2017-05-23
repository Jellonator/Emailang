use user::*;
use interpreter::Interpreter;
use mail::Mail;
use std::char;

fn func(user: &User, inter: &mut Interpreter, mail: &Mail) {
	match mail.subject.as_ref() {
		"add" => {
			let mut is_okay = true;
			let sum = mail.attachments
				.iter()
				.filter_map(|v| {
					match v.parse::<i64>() {
						Ok(val) => Some(val),
						_ => {
							is_okay = false;
							None
						}
					}
				})
				.fold(0, |acc, x| acc + x);
			if is_okay {
				inter.mail(Mail::create(mail.to.clone(), mail.from.clone(), &mail.message, &sum.to_string()));
			} else {
				inter.mail(Mail::create(mail.to.clone(), mail.from.clone(), &mail.message, ""));
			}
		},
		"mul" => {
			let mut is_okay = true;
			let sum = mail.attachments
				.iter()
				.filter_map(|v| {
					match v.parse::<i64>() {
						Ok(val) => Some(val),
						_ => {
							is_okay = false;
							None
						}
					}
				})
				.fold(1, |acc, x| acc * x);
			if is_okay {
				inter.mail(Mail::create(mail.to.clone(), mail.from.clone(), &mail.message, &sum.to_string()));
			} else {
				inter.mail(Mail::create(mail.to.clone(), mail.from.clone(), &mail.message, ""));
			}
		},
		"div" => {
			let mut is_okay = true;
			let base = mail.attachments
				.get(0)
				.map(|v| v.parse::<i64>().ok())
				.unwrap_or(Some(0));
			let base = match base {
				Some(val) => val,
				None => {
					is_okay = false;
					0
				}
			};
			let sum = mail.attachments[1..]
				.iter()
				.filter_map(|v| {
					match v.parse::<i64>() {
						Ok(val) => Some(val),
						_ => {
							is_okay = false;
							None
						}
					}
				})
				.fold(base, |acc, x| acc / x);
			if is_okay {
				inter.mail(Mail::create(mail.to.clone(), mail.from.clone(), &mail.message, &sum.to_string()));
			} else {
				inter.mail(Mail::create(mail.to.clone(), mail.from.clone(), &mail.message, ""));
			}
		},
		"ord" => {
			let ords = mail.attachments
				.iter()
				.map(|v|v.as_str())
				.collect::<String>()
				.chars()
				.map(|v|v.to_string())
				.collect::<Vec<String>>();
			let mut retmail = Mail::create(
				mail.to.clone(),
				mail.from.clone(),
				&mail.message,
				&ords.get(0).map(|v|v.as_str()).unwrap_or("0"));
			for val in ords {
				retmail.attach(&val);
			}
			inter.mail(retmail);
		},
		"char" => {
			// NULL character can represent errors in this case
			let chars = mail.attachments
				.iter()
				.map(|v|v.parse::<u32>().unwrap_or(0))
				.map(|v|char::from_u32(v).unwrap_or('\0'))
				.collect::<String>();
			inter.mail(Mail::create(mail.to.clone(), mail.from.clone(), &mail.message, &chars));
		},
		o => println!("Bad math function {}!", o)
	}
}

pub fn create() -> UserDef {
	UserDef::create_def_external(Box::new(func))
}
