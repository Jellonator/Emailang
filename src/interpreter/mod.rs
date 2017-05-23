#![allow(dead_code)]
use instruction::Instruction;
use server::Server;
use mail::Mail;
use user::*;
use environment::Environment;
mod std;
use std::collections::HashMap;

pub struct Interpreter {
	servers: HashMap<String, Server>,
	pending: Vec<Mail>,
	users_to_add: Vec<(String, User)>,
	servers_to_add: Vec<String>,
}

impl Interpreter {
	pub fn new() -> Interpreter {
		let mut inter = Interpreter {
			servers: HashMap::new(),
			pending: Vec::new(),
			users_to_add: Vec::new(),
			servers_to_add: Vec::new(),
		};

		std::create_std_lib(&mut inter);
		inter
	}

	pub fn add_user(&mut self, server: &str, user: &User) {
		self.users_to_add.push((server.to_string(), user.clone()));
	}

	pub fn add_server(&mut self, server: &str) {
		self.servers_to_add.push(server.to_string());
	}

	pub fn mail(&mut self, mail: Mail) {
		// println!("Sending mail {} to {:?}!", mail.subject, mail.to);
		self.pending.push(mail);
	}

	fn get_server(&mut self, name: &str) -> Option<&mut Server> {
		self.servers.get_mut(name)
	}

	fn handle_sent_mail(&mut self, mail: &Mail) {
		let tuser = &mail.to.0;
		let tserver = &mail.to.1;

		let selfhack = self as *mut Interpreter;
		let selfhack = unsafe {&mut*selfhack};

		let mut serv = match self.get_server(&tserver) {
			Some(val) => val,
			None => return
		};

		let mut user = match serv.get_user_mut(&tuser) {
			Some(val) => val,
			None => return
		};

		// Just a note that theoretically this should be safe
		user.send(selfhack/*huehuehue*/, &mail);
	}

	pub fn handle_pending(&mut self) -> bool {
		if self.servers_to_add.len() == 0 &&
		   self.users_to_add.len() == 0 &&
		   self.pending.len() == 0 {
			return false;
		}

		for server_name in self.servers_to_add.drain(..) {
			self.servers.insert(server_name, Server{
				users: HashMap::new()
			});
		}
		let users = self.users_to_add.split_off(0);
		for def in users {
			let mut serv = self.get_server(&def.0).unwrap();
			let name = def.1.env.path.0.to_string();
			serv.add_user(name, def.1);
		}
		let mail = self.pending.split_off(0);
		for m in mail {
			self.handle_sent_mail(&m);
		}

		return true;
	}

	pub fn run(&mut self, instructions: &Vec<Instruction>, env: &mut Environment) {
		let mut i = 0;
		loop {
			if i >= instructions.len() {
				break;
			}
			let inst = &instructions[i];
			i = i + 1;
			inst.call(self, env);
		}
	}

	pub fn execute(&mut self, instructions: &Vec<Instruction>) {
		self.handle_pending();
		let mut env = Environment::new_anon();
		self.run(instructions, &mut env);
		while self.handle_pending() {}
	}
}
