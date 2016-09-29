#![allow(dead_code)]
use instruction::Instruction;
use server::Server;
use mail::Mail;
use user::*;

pub struct Interpreter {
	servers: Vec<Server>,
	pending: Vec<Mail>,
	users_to_add: Vec<(String, User)>,
	servers_to_add: Vec<String>,
}

impl Interpreter {
	pub fn new() -> Interpreter {
		let mut inter = Interpreter {
			servers: Vec::new(),
			pending: Vec::new(),
			users_to_add: Vec::new(),
			servers_to_add: Vec::new(),
		};
		inter.add_server("std");
		inter.add_user("std",
			&User::create_user_external("print", Box::new(|_, m| println!("{}", m.message)))
		);

		inter.handle_pending();
		inter
	}

	pub fn add_user(&mut self, server: &str, user: &User) {
		self.users_to_add.push((server.to_string(), user.clone()));
	}

	pub fn add_server(&mut self, server: &str) {
		self.servers_to_add.push(server.to_string());
	}

	pub fn mail(&mut self, mail: &Mail) {
		self.pending.push(mail.clone());
	}

	fn get_server(&mut self, name: &str) -> Option<&mut Server> {
		for serv in self.servers.iter_mut() {
			if &serv.name == name {
				return Some(serv);
			}
		}
		None
	}

	fn send_mail(&mut self, mail: &Mail) {
		let tserver = &mail.to.0;
		let tuser = &mail.to.1;

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
			self.servers.push(Server{
				name: server_name,
				users: Vec::new()
			});
		}
		let users = self.users_to_add.split_off(0);
		for def in users {
			let mut serv = self.get_server(&def.0).unwrap();
			serv.users.push(def.1);
		}
		let mail = self.pending.split_off(0);
		for m in mail {
			self.send_mail(&m);
		}

		return true;
	}

	pub fn run(&mut self, instructions: &Vec<Instruction>) {
		self.handle_pending();
		let mut i = 0;
		loop {
			if i >= instructions.len() {
				break;
			}
			let inst = &instructions[i];
			i = i + 1;
			inst.call(self);
		}
		while self.handle_pending() {}
	}
}
