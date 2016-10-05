use user::User;
use interpreter::Interpreter;
use mail::Mail;

fn func(user: &User, inter: &mut Interpreter, mail: &Mail) {
	match mail.subject.as_ref() {
		"add" => {
			let sum = mail.attachments
				.iter()
				.map(|v| v.parse::<i64>().unwrap())
				.fold(0, |acc, x| acc + x);
			inter.send_mail(&user.create_mail(mail.from.clone(), &mail.message, &sum.to_string()));
		},
		"mul" => {
			let sum = mail.attachments
				.iter()
				.map(|v| v.parse::<i64>().unwrap())
				.fold(1, |acc, x| acc * x);
			inter.send_mail(&user.create_mail(mail.from.clone(), &mail.message, &sum.to_string()));
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
			inter.send_mail(&user.create_mail(mail.from.clone(), &mail.message, &sum.to_string()));
		},
		o => println!("Bad loop function {}!", o)
	}
}

pub fn create() -> User {
	User::create_user_external("math", Box::new(func))
}
