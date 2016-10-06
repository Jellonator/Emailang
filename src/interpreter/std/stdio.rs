use user::User;
use interpreter::Interpreter;
use mail::Mail;

fn func(_: &User, _: &mut Interpreter, mail: &Mail) {
	match mail.subject.as_ref() {
		"print" => {
			print!("{}", mail.message);
			for val in &mail.attachments {
				print!(" {}", val);
			}
		},
		"println" => {
			print!("{}", mail.message);
			for val in &mail.attachments {
				print!(" {}", val);
			}
			print!("\n");
		},
		o => println!("Bad io function {}!", o)
	}
}

pub fn create() -> User {
	User::create_user_external("io", Box::new(func))
}
