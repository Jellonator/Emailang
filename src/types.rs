use user::UserPath;

pub enum Type {
	Null,
	Text(String),
	Server(String),
	UserPath(UserPath)
}
