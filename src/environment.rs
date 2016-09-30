use types::Type;
use std::collections::HashMap;

pub struct Environment {
	pub data: HashMap<String, Type>
}

impl Environment {
	pub fn new() -> Environment {
		Environment {
			data: HashMap::new()
		}
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
