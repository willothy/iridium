use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

use crate::{
    assembler::{program::parse_program, Assembler},
    opcode::OpCode,
    vm::VM,
};

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
        let mut asm = Assembler::new();
        loop {
            if *self.vm.read_pc() < self.vm.program_len() {
                self.vm.run();
            }
            print!(">>> ");
            output.flush().unwrap_or_else(|_| println!(""));
            buffer.clear();
            input.read_line(&mut buffer).expect("Failed to read stdin");

            if buffer.starts_with("!") {
                if let Err(ShouldExit) = self.execute_command(&buffer[1..]) {
                    return Ok(());
                };
                continue;
            }

            let program = asm.assemble(&buffer)?;
            self.vm.add_program(program);
            self.command_buffer.push(buffer.clone());

            self.vm.run();
        }
    }

    fn execute_command(&mut self, cmd: &str) -> Result<(), ShouldExit> {
        match cmd.trim() {
            "quit" => {
                println!("Bye!");
                return Err(ShouldExit);
            }
            "open" => {
                print!("Please enter the path to the file you wish to load: ");
                std::io::stdout().flush().expect("Unable to flush stdout");
                let mut tmp = String::new();
                std::io::stdin()
                    .read_line(&mut tmp)
                    .expect("Unable to read line from user");
                let tmp = tmp.trim();
                let filename = Path::new(&tmp);
                let mut f = match File::open(Path::new(&filename)) {
                    Ok(f) => f,
                    Err(_) => {
                        println!("Unable to open file");
                        return Ok(());
                    }
                };
                let mut contents = String::new();
                if let Err(e) = f.read_to_string(&mut contents) {
                    println!("Unable to read file: {}", e);
                    return Ok(());
                };
                let program = match parse_program(&contents) {
                    Ok(program) => program,
                    Err(e) => {
                        println!("Unable to parse input: {:?}", e);
                        return Ok(());
                    }
                };
                self.vm.add_program(program.to_bytes());
                Ok(())
            }
            "state" => {
                println!("{}", self.vm);
                return Ok(());
            }
            "bytecode" => {
                println!("Bytecode:");
                let mut buffer = String::new();
                for instruction in self.vm.read_program() {
                    buffer.push_str(&format!("{:04x} ", instruction));
                    if buffer.len() >= 16 {
                        println!("{}", buffer);
                        buffer.clear();
                    }
                }

                println!("End of Bytecode");
                return Ok(());
            }
            "reg" => {
                println!("Register File: ");
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
                println!("End of Register File");
                return Ok(());
            }
            "program" => {
                println!("Program:");
                let mut instruction = Vec::new();
                for byte in self.vm.read_program().iter() {
                    if instruction.len() == 4 {
                        println!(
                            "{} {:04} {:04} {:04}",
                            OpCode::from(instruction[0]).padded(),
                            instruction[1],
                            instruction[2],
                            instruction[3]
                        );
                        instruction.clear();
                    }
                    instruction.push(*byte);
                }
                println!("End of Program");
                return Ok(());
            }
            _ => {
                println!("Invalid command.");
                return Ok(());
            }
        }
    }
}
