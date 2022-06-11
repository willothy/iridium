use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

use crate::{
    assembler::Assembler,
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

            let program = match asm.assemble(&buffer) {
                Ok(program) => program,
                Err(e) => {
                    println!("Errors found: ");
                    e.iter().for_each(|e| println!("{}", e));
                    self.vm.reset();
                    self.command_buffer.clear();
                    continue;
                }
            };
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
            "reset" => {
                self.vm.reset();
                println!("Reset complete.");
                return Ok(());
            }
            "open" => {
                print!("Please enter the path to the file you wish to load: ");
                std::io::stdout().flush().expect("Unable to flush stdout");
                let mut tmp = String::new();
                std::io::stdin()
                    .read_line(&mut tmp)
                    .expect("Unable to read line from user");
                let filename = Path::new(tmp.trim());
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
                let program = match Assembler::new().assemble(&contents) {
                    Ok(program) => program,
                    Err(e) => {
                        println!("Errors found: ");
                        e.iter().for_each(|e| println!("{}", e));
                        return Ok(())
                    }
                };
                self.vm.add_program(program);
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
                //for byte in self.vm.read_program().iter() {
                for byte in self.command_buffer.iter().map(|s| s.as_str()) {
                    if instruction.len() == 4 {
                        println!(
                            "{} {:04} {:04} {:04}",
                            /* match OpCode::from(instruction[0]) {
                                Ok(opcode) => opcode,
                                Err(_) => OpCode::IGL,
                            }.padded(), */
                            OpCode::from_string(instruction[0]),
                            instruction[1],
                            instruction[2],
                            instruction[3]
                        );
                        instruction.clear();
                    }
                    instruction.push(byte); // *byte
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
