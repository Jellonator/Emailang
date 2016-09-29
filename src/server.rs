use user::User;

pub struct Server {
	pub name: String,
	pub users: Vec<User>
}

impl Server {
	pub fn get_user(&self, name: &str) -> Option<&User> {
		for u in self.users.iter() {
			if &u.name == name {
				return Some(u)
			}
		}
		None
	}

	pub fn get_user_mut(&mut self, name: &str) -> Option<&mut User> {
		for u in self.users.iter_mut() {
			if &u.name == name {
				return Some(u)
			}
		}
		None
	}
}
