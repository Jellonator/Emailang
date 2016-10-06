use user::User;
use interpreter::Interpreter;
use mail::Mail;
use std::io::{self, BufRead, Write};

fn func(_: &User, inter: &mut Interpreter, mail: &Mail) {
	match mail.subject.as_ref() {
		"print" => {
			print!("{}", mail.message);
			for val in &mail.attachments {
				print!(" {}", val);
			}
			io::stdout().flush().unwrap();
		},
		"println" => {
			print!("{}", mail.message);
			for val in &mail.attachments {
				print!(" {}", val);
			}
			print!("\n");
		},
		"input" => {
			let mut line = String::new();
			let stdin = io::stdin();
			stdin.lock().read_line(&mut line).unwrap();
			mail.return_mail(inter, &mail.message, line.trim_right(), Vec::new());
		},
		o => println!("Bad io function {}!", o)
	}
}

pub fn create() -> User {
	User::create_user_external("io", Box::new(func))
}
