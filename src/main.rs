mod statement;

use std::io::{self, Write, BufRead};
use std::process;
use statement::{MetaCommandResult, PrepareResult, Statement, StatementType};

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

fn do_mata_command(input_buffer: &InputBuffer) -> MetaCommandResult {
	if input_buffer.buffer.trim() == ".exit" {
		return MetaCommandResult::MetaCommandSuccess;
	} else {
		return MetaCommandResult::MetaCommandUnrecognizedCommand;
	}
}

fn prepare_statement(input_buffer: &InputBuffer, statement: &mut Statement) -> PrepareResult {
	if input_buffer.buffer.contains("insert") {
		statement._type = StatementType::StatementInsert;
		return PrepareResult::PrepareSuccess;
	}
	if input_buffer.buffer.contains("select") {
		statement._type = StatementType::StatementSelect;
		return PrepareResult::PrepareSuccess;
	}
	return PrepareResult::PrepareUnrecognizedStatement;
}

fn execute_statement(statement: Statement) {
	match statement._type {
		StatementType::StatementInsert => println!("Insert successfully"),
		StatementType::StatementSelect => println!("Select successfully"),
	}
}

fn main() {
	let mut input_buffer = InputBuffer::new_input_buffer();
	loop {
		print_prompt();
		input_buffer.buffer = io::stdin().lock().lines().next().unwrap().unwrap();
	
		if input_buffer.buffer.chars().nth(0).unwrap() == '.' {
			match do_mata_command(&input_buffer) {
				MetaCommandResult::MetaCommandSuccess => {
					println!("Thanks for using zlite.");
					process::exit(0);
				},
				MetaCommandResult::MetaCommandUnrecognizedCommand => {
					println!("Unrecognized command {:?}\n", input_buffer.buffer);
					continue;
				}
			}
		}

		let mut _statement = Statement { _type: None, rowToInsert: None };
		match prepare_statement(&input_buffer, &mut _statement) {
			PrepareResult::PrepareSuccess => execute_statement(_statement),
			PrepareResult::PrepareUnrecognizedStatement => {
				println!("Unrecognized keyword at start of {:?}.", input_buffer.buffer);
				continue;
			}
		}

	}
}
