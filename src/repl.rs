use std::io::{self, Write};

use crate::vm::VM;

struct ShouldExit;

pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM,
}

impl REPL {
    pub fn new() -> Self {
        Self {
            command_buffer: Vec::new(),
            vm: VM::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = String::new();
        let mut output = io::stdout();
        let input = io::stdin();
        loop {
            print!(">>> ");
            output.flush().unwrap_or_else(|_| println!(""));
            input.read_line(&mut buffer).expect("Failed to read stdin");
            if buffer.starts_with("!") {
                if let Err(ShouldExit) = self.execute_command(&buffer[1..]) {
                    return Ok(());
                };
                buffer.clear();
                continue;
            }
            /*let mut split = buffer.split_whitespace();
            let opcode_str = split.next().ok_or("No instruction name")?.trim().to_owned();
            let args = split.collect::<Vec<&str>>();

            let opcode = OpCode::from(opcode_str.clone());

            if let OpCode::IGL = opcode {
                println!("Unknown instruction {}", opcode_str);
                buffer.clear();
                continue;
            }

            let instruction = Instruction::new(opcode, self.parse_hex(args)?);*/
            let program = crate::assembler::parser::program(&buffer)?.1;

            self.vm.add_program(program.to_bytes());
            self.command_buffer.push(buffer.clone());

            self.vm.run();

            buffer.clear();
        }
    }

    fn execute_command(&mut self, cmd: &str) -> Result<(), ShouldExit> {
        match cmd.trim() {
            "quit" => {
                println!("Bye!");
                return Err(ShouldExit);
            }
            "state" => {
                println!("{}", self.vm);
                return Ok(());
            }
            "bytecode" => {
                println!("Program:");
                let mut buffer = String::new();
                for instruction in self.vm.read_program() {
                    buffer.push_str(&format!("{:04} ", instruction));
                    if buffer.len() >= 16 {
                        println!("{}", buffer);
                        buffer.clear();
                    }
                }
                println!("End of Program Listing");
                return Ok(());
            }
            "registers" => {
                println!("Listing registers and all contents: ");
                //println!("{:#?}", self.vm.read_registers());
                let mut buffer = String::from("[ ");
                for (i, register) in self.vm.read_registers().iter().enumerate() {
                    buffer.push_str(&format!("{register}"));
                    if i < 31 {
                        buffer.push_str(", ");
                    }
                }
                buffer.push_str(" ]");
                println!("{}", buffer);
                println!("End of Register Listing");
                return Ok(());
            }
            "history" => {
                println!("Instruction History:");
                for instruction in &self.command_buffer {
                    println!("{}", instruction);
                }
                println!("End of Instruction History");
                return Ok(());
            }
            _ => {
                println!("Invalid command.");
                return Ok(());
            }
        }
    }
}
