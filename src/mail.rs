use user::UserPath;
use interpreter::Interpreter;

#[derive(Clone)]
pub struct Mail {
	pub subject: String,
	pub message: String,
	pub from: UserPath,
	pub to: UserPath,
	pub attachments: Vec<String>
}

impl Mail {
	pub fn attach(&mut self, val: &str) {
		self.attachments.push(val.to_string())
	}

	pub fn create(from: UserPath, to: UserPath, subject: &str, message: &str) -> Mail {
		Mail {
			from: from,
			to: to,
			subject: subject.to_string(),
			message: message.to_string(),
			attachments: Vec::new()
		}
	}

	pub fn return_mail(&self, inter: &mut Interpreter, subject: &str, message: &str, attachments: Vec<String>) {
		inter.mail(Mail {
			from: self.to.clone(),
			to: self.from.clone(),
			subject: subject.to_string(),
			message: message.to_string(),
			attachments: attachments
		});
	}
}

#[derive(Clone)]
pub struct Draft {
	pub subject: String,
	pub message: String,
	pub attachments: Vec<String>
}
