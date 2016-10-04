use user::UserPath;

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
}

#[derive(Clone)]
pub struct Draft {
	pub subject: String,
	pub message: String,
	pub attachments: Vec<String>
}
