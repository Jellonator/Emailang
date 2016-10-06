use user::User;
use interpreter::Interpreter;
use mail::Mail;

fn func(user: &User, inter: &mut Interpreter, mail: &Mail) {
	match mail.subject.as_ref() {
		"iterate" => { // Iterate through all attachments
			for a in &mail.attachments {
				inter.mail(user.create_mail(mail.from.clone(), &mail.message, &a));
			}
		},
		o => println!("Bad loop function {}!", o)
	}
}

pub fn create() -> User {
	User::create_user_external("loop", Box::new(func))
}
