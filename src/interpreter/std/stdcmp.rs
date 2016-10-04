use user::User;
use interpreter::Interpreter;
use mail::Mail;

fn func(user: &User, inter: &mut Interpreter, mail: &Mail) {
	match mail.subject.as_ref() {
		"eq" => {
			if mail.attachments.get(0) == mail.attachments.get(1) {
				inter.send_mail(&user.create_mail(mail.from.clone(), &mail.message, "true"));
			} else {
				inter.send_mail(&user.create_mail(mail.from.clone(), &mail.message, "false"));
			}
		},
		o => println!("Bad loop function {}!", o)
	}
}

pub fn create() -> User {
	User::create_user_external("cmp", Box::new(func))
}
