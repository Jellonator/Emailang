use interpreter::Interpreter;
mod stdcmp;
mod stdio;
mod stdloop;

pub fn create_std_lib(inter: &mut Interpreter) {
	inter.add_server("std.com");

	// IO Functions
	inter.add_user("std.com", &stdio::create());

	// Loop Constructs
	inter.add_user("std.com", &stdloop::create());

	// Boolean tests
	inter.add_user("std.com", &stdcmp::create());

	inter.handle_pending();
}
