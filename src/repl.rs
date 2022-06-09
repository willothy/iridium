use std::{
    io::{self, Write},
    num::ParseIntError,
};

use crate::{
    instruction::{Instruction, OpCode, Tou8Vec},
    vm::VM,
};

struct ShouldExit;

pub struct REPL {
    command_buffer: Vec<Instruction>,
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
            let mut split = buffer.split_whitespace();
            let opcode_str = split.next().ok_or("No instruction name")?.trim().to_owned();
            let args = split.collect::<Vec<&str>>();

            let opcode = OpCode::from(opcode_str.clone());

            if let OpCode::IGL = opcode {
                println!("Unknown instruction {}", opcode_str);
                buffer.clear();
                continue;
            }

            let instruction = Instruction::new(opcode, self.parse_hex(args)?);

            self.vm.add_command(instruction.to_u8_vec());
            self.command_buffer.push(instruction);

            if let Err(err) = self.vm.run_once() {
                println!("{}", err);
            }

            buffer.clear();
        }
    }

    fn parse_hex(&mut self, split: Vec<&str>) -> Result<Vec<u8>, ParseIntError> {
        let mut results: Vec<u8> = vec![];
        for hex_string in split {
            results.push(u8::from_str_radix(&hex_string, 16)?);
        }
        Ok(results)
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
                    buffer.push_str(&format!("{:02} ", instruction));
                    if buffer.len() >= 12 {
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
