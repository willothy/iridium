use std::process::ExitCode;

#[macro_use]
extern crate enum_primitive;
extern crate num_traits;
//extern crate num;

mod assembler;
mod opcode;
mod repl;
mod logger;
mod vm;

fn main() -> ExitCode {
    println!("Welcome to the VM!");
    let mut repl = repl::REPL::new();
    return match repl.run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            println!("Error: {}", e);
            ExitCode::FAILURE
        }
    };
}
