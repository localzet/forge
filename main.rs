mod lexer;
mod parser;
mod ast;
mod types;
mod compiler;
mod vm;

use std::env;
use std::fs;

fn main() {
	let mut args: Vec<String> = env::args().collect();
	if args.len() < 3 {
		eprintln!("Usage: forge <run|build> <file>");
		return;
	}
	let cmd = &args[1];
	let file = &args[2];

	let src = fs::read_to_string(file).expect("Failed to read source file");

	let mut lexer = lexer::Lexer::new(&src);
	let tokens = lexer.tokenize();

	let mut parser = parser::Parser::new(tokens);
	let program = parser.parse_program();

	// type checking (basic)
	let mut checker = types::checker::TypeChecker::new();
	for func in &program.functions {
		for stmt in &func.body {
			checker.check_statement(stmt);
		}
	}

	let mut compiler = compiler::Compiler::new();
	compiler.compile(program);

	if cmd == "run" {
		let mut vm = vm::VM::new(compiler.functions);
		vm.run("main");
	} else if cmd == "build" {
		println!("Build complete (bytecode in memory). Use run to execute.");
	} else {
		eprintln!("Unknown command: {}", cmd);
	}
}
