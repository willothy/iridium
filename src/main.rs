use std::process::ExitCode;

mod instruction;
mod vm;

fn main() -> ExitCode {
    println!("{}", instruction::OpCode::HLT as u8);
    return ExitCode::SUCCESS;
}
