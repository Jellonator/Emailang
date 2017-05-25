use user::*;
use interpreter::Interpreter;
use mail::Mail;

fn func(_: &User, inter: &mut Interpreter, mail: &Mail) {
	match mail.subject.as_ref() {
		"iterate" => { // Iterate through all attachments
			for a in &mail.attachments {
				inter.mail(Mail::create(mail.to.clone(), mail.from.clone(), &mail.message, &a));
			}
		},
		o => println!("Bad loop function {}!", o)
	}
}

pub fn create() -> UserDef {
	UserDef::create_def_external(Box::new(func))
}
