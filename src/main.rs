use std::{io::{self, stdout, Write}, fs, env};

use vm::{Vm, InterpretResult};

mod chunk;
mod value;
mod opcode;
mod compiler;
mod scanner;
mod token;
mod vm;

fn run_file(file_name: String) {
    let file_path = format!("examples/{}", file_name);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut vm = Vm::new();
    let result = vm.interpret(&contents);

    match result {
        InterpretResult::InterpretCompileError=> std::process::exit(65),
        InterpretResult::InterpretRuntimeError => std::process::exit(70),
        InterpretResult::InterpretOk => std::process::exit(0)
    }
}

fn repl() {
    let stdin = io::stdin();
    let mut vm = Vm::new();
    print!("> ");
    stdout().flush().unwrap();

    for line in stdin.lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            vm.interpret(&line);
        } else {
            break;
        }

        print!("> ");
        stdout().flush().unwrap();
    }
}

fn main() {
    let mut vm = Vm::new();
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        repl();
    } else {
        let file_name = &args[1];
        run_file(file_name.to_string());
    }

}
