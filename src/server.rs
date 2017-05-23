use user::User;
use std::collections::HashMap;

pub struct Server {
	pub users: HashMap<String, User>
}

impl Server {
	pub fn get_user(&self, name: &str) -> Option<&User> {
		self.users.get(name)
	}

	pub fn get_user_mut(&mut self, name: &str) -> Option<&mut User> {
		self.users.get_mut(name)
	}

	pub fn add_user(&mut self, name: String, user: User) {
		self.users.insert(name, user);
	}
}
