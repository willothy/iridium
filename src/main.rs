use std::process::ExitCode;

mod assembler;
mod opcode;
mod repl;
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
