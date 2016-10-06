use user::User;
use interpreter::Interpreter;
use mail::Mail;
use std::char;

fn func(user: &User, inter: &mut Interpreter, mail: &Mail) {
	match mail.subject.as_ref() {
		"add" => {
			let sum = mail.attachments
				.iter()
				.map(|v| v.parse::<i64>().unwrap())
				.fold(0, |acc, x| acc + x);
			inter.mail(user.create_mail(mail.from.clone(), &mail.message, &sum.to_string()));
		},
		"mul" => {
			let sum = mail.attachments
				.iter()
				.map(|v| v.parse::<i64>().unwrap())
				.fold(1, |acc, x| acc * x);
			inter.mail(user.create_mail(mail.from.clone(), &mail.message, &sum.to_string()));
		},
		"div" => {
			let base = mail.attachments
				.get(0)
				.map(|v| v.parse::<i64>().unwrap())
				.unwrap_or(0);
			let sum = mail.attachments[1..]
				.iter()
				.map(|v| v.parse::<i64>().unwrap())
				.fold(base, |acc, x| acc / x);
			inter.mail(user.create_mail(mail.from.clone(), &mail.message, &sum.to_string()));
		},
		"ord" => {
			let ords = mail.attachments
				.iter()
				.map(|v|v.as_str())
				.collect::<String>()
				.chars()
				.map(|v|v.to_string())
				.collect::<Vec<String>>();
			let mut retmail = user.create_mail(
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
			inter.mail(user.create_mail(mail.from.clone(), &mail.message, &chars));
		},
		o => println!("Bad math function {}!", o)
	}
}

pub fn create() -> User {
	User::create_user_external("math", Box::new(func))
}
