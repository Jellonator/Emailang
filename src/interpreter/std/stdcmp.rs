use user::*;
use interpreter::Interpreter;
use mail::Mail;

fn func(_: &User, inter: &mut Interpreter, mail: &Mail) {
	match mail.subject.as_ref() {
		"eq" => {
			inter.mail(Mail::create(mail.to.clone(), mail.from.clone(), &mail.message,
				match mail.attachments.get(0) == mail.attachments.get(1) {
					true => "true",
					false => "false"
				}
			));
		},
		"neq" => {
			inter.mail(Mail::create(mail.to.clone(), mail.from.clone(), &mail.message,
				match mail.attachments.get(0) != mail.attachments.get(1) {
					true => "true",
					false => "false"
				}
			));
		},
		o => println!("Bad loop function {}!", o)
	}
}

pub fn create() -> UserDef {
	UserDef::create_def_external(Box::new(func))
}
