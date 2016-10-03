use types::Type;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Environment {
	pub data: HashMap<String, Type>,
	pub username: String,
	pub server: String,
}

impl Environment {
	pub fn new(username: &str, server: &str) -> Environment {
		Environment {
			data: HashMap::new(),
			username: username.to_string(),
			server: server.to_string()
		}
	}

	pub fn has(&self, key: &str) -> bool {
		self.data.contains_key(key)
	}

	pub fn new_anon() -> Environment {
		Environment::new("Anonymous", "anon")
	}

	pub fn set(&mut self, key: &str, value: Type) {
		if let Type::Null = value {
			self.data.remove(key);
		} else {
			self.data.insert(key.to_string(), value);
		}
	}

	pub fn get(&self, key: &str) -> Type {
		match self.data.get(key) {
			Some(val) => val.clone(),
			None => Type::Null
		}
	}
}
