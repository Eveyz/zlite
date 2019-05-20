mod constants;
mod table;
mod statement;

use std::mem;
use std::io::{self, Write};
use std::process;
use statement::{MetaCommandResult, PrepareResult, Statement, StatementType};
#[macro_use] extern crate scan_fmt;

// cargo watch -x run

// struct InputBufferT {
// 	buffer: String,
// 	buffer_length: usize,
// 	input_lenth: isize,
// }
// type InputBuffer = InputBufferT;

// impl InputBuffer {
// 	fn new_input_buffer() -> InputBuffer {
// 		return InputBuffer { buffer: String::new() };
// 	}
// }

fn print_prompt() {
	print!("zlite > ");
	io::stdout().flush().unwrap();
}

fn do_mata_command(input_buffer: &String) -> MetaCommandResult {
	if input_buffer.trim() == ".exit" {
		return MetaCommandResult::MetaCommandSuccess;
	} else {
		return MetaCommandResult::MetaCommandUnrecognizedCommand;
	}
}

fn prepare_statement(input_buffer: &String, statement: &mut Statement) -> PrepareResult {
	if input_buffer.contains("insert") {
		match scan_fmt!(input_buffer, "{} {} {} {}", String, usize, String, String) {
			Ok(_) => {
				statement.statement_type = Some(StatementType::StatementInsert);
				return PrepareResult::PrepareSuccess;
			},
			Err(_) => {
				return PrepareResult::PrepareSyntaxError;
			}
		}
	}
	if input_buffer.contains("select") {
		statement.statement_type = Some(StatementType::StatementSelect);
		return PrepareResult::PrepareSuccess;
	}
	return PrepareResult::PrepareUnrecognizedStatement;
}

fn execute_statement(statement: Statement) {
	match statement.statement_type {
		Some(StatementType::StatementInsert) => println!("Insert successfully"),
		Some(StatementType::StatementSelect) => println!("Select successfully"),
		None => ()
	}
}

fn main() {
	let mut input_buffer = String::new();
	loop {
		print_prompt();
		match io::stdin().read_line(&mut input_buffer) {
			Ok(_b) => {
				// println!("{} bytes read. The result is {}", b, input_buffer);
				
				// handle meta command
				if input_buffer.chars().nth(0).unwrap() == '.' {
					match do_mata_command(&input_buffer) {
						MetaCommandResult::MetaCommandSuccess => {
							println!("Thanks for using zlite.");
							process::exit(0);
						},
						MetaCommandResult::MetaCommandUnrecognizedCommand => {
							println!("Unrecognized command {}", input_buffer);
						}
					}
				}
				
				// handle sql statements
				let mut _statement = Statement::new();
				match prepare_statement(&input_buffer, &mut _statement) {
					PrepareResult::PrepareSuccess => execute_statement(_statement),
					PrepareResult::PrepareSyntaxError => {
						println!("Syntax error.");
					},
					PrepareResult::PrepareUnrecognizedStatement => {
						println!("Unrecognized keyword at start of {}", input_buffer);
					}
				}
			},
			Err(_e) => {
				println!("Sorry. Cannot parse token.");
			}
		}
		input_buffer.clear();
		assert!(input_buffer.is_empty());
		// input_buffer.buffer = io::stdin().lock().lines().next().unwrap().unwrap();
	
	}
}
