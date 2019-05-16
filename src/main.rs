use std::io::{self, Write, BufRead};
use std::process;

// cargo watch -x run

struct InputBufferT {
	buffer: String,
	// buffer_length: usize,
	// input_lenth: isize,
}
type InputBuffer = InputBufferT;

impl InputBuffer {
	fn new_input_buffer() -> InputBuffer {
		return InputBuffer { buffer: String::new() };
	}
}

fn print_prompt() {
	print!("zlite > ");
	io::stdout().flush().unwrap();
}

fn main() {
	let mut input_buffer = InputBuffer::new_input_buffer();
	loop {
		print_prompt();
		input_buffer.buffer = io::stdin().lock().lines().next().unwrap().unwrap();
	
		if input_buffer.buffer.trim() == ".exit" {
			println!("Thanks for using zlite.");
			process::exit(0);
		} else {
			println!("input {:?}", input_buffer.buffer);
		}
	}
}
