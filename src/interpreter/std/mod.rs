use interpreter::Interpreter;
mod stdcmp;
mod stdio;
mod stdloop;
mod stdmath;

pub fn create_std_lib(inter: &mut Interpreter) {
	inter.add_server("std.com");

	// IO Functions
	inter.add_user("io", "std.com", &stdio::create());

	// Loop Constructs
	inter.add_user("loop", "std.com", &stdloop::create());

	// Boolean tests
	inter.add_user("cmp", "std.com", &stdcmp::create());

	// Math
	inter.add_user("math", "std.com", &stdmath::create());

	inter.handle_pending();
}
