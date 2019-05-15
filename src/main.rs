use std::io::{self, Read};

struct InputBufferT {
	buffer: String,
	buffer_length: usize,
	input_lenth: isize,
}
type InputBuffer = InputBufferT;

impl InputBuffer {
	fn new_input_buffer() -> InputBuffer {
		return InputBuffer { buffer: String::new(), buffer_length: 0, input_lenth: 0 };
	}
}

fn print_prompt() {
	println!("zlite > ");
}

fn read_input(input_buffer: &mut InputBuffer) -> io::Result<()> {
	io::stdin().read_to_string(&mut input_buffer.buffer)?;
	return Ok(());
}

fn main() {
	let mut input_buffer = InputBuffer::new_input_buffer();
	loop {
		print_prompt();
		read_input(&mut input_buffer);

		if input_buffer.buffer == ".exit" {
			break;
		} else {
			println!("Input {:?}", input_buffer.buffer);
		}
	}
}
