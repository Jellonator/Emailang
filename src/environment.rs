use types::Type;
use std::collections::HashMap;
use user::UserPath;

#[derive(Clone, Debug)]
pub struct Environment {
	pub data: HashMap<String, Type>,
}

impl Environment {
	pub fn new() -> Environment {
		Environment {
			data: HashMap::new()
		}
	}

	pub fn has(&self, key: &str) -> bool {
		self.data.contains_key(key)
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
