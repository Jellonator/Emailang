use user::UserPath;

#[derive(Clone)]
pub struct Mail {
	pub subject: String,
	pub message: String,
	// pub from: UserPath,
	pub to: UserPath
}

#[derive(Clone)]
pub struct Draft {
	pub subject: String,
	pub message: String,
}
