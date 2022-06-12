use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

use crate::{
    assembler::{Assembler, AssemblerError},
    vm::VM,
};

use crossterm::{execute, style::Print, cursor::{MoveToPreviousLine, position, MoveTo}, terminal::{ClearType, Clear}, queue};

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

    fn assemble_from<'asm>(&mut self, asm: &'asm mut Assembler, program: &str) -> Result<&'asm mut Vec<u8>, &'asm Vec<AssemblerError>> {
        match asm.assemble(program) {
            Ok(bytecode) => Ok(bytecode),
            Err(e) => {
                self.vm.reset();
                self.command_buffer.clear();
                Err(e)
            }
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = String::new();
        let mut output = io::stdout();
        let input = io::stdin();
        let mut asm = Assembler::new();
        execute!(output, crossterm::cursor::EnableBlinking).unwrap_or_else(|_| {});
        loop {
            execute!(output, MoveTo(0, position().unwrap().1), Clear(ClearType::CurrentLine), Print(">>> ")).unwrap_or_else(|_| {});
            input.read_line(&mut buffer).expect("Failed to read stdin");
            execute!(output, MoveToPreviousLine(1)).unwrap_or_else(|_| {});
            if buffer.starts_with("!") || buffer.starts_with("\n") {
                if let Err(ShouldExit) = self.execute_command(&buffer[1..]) {
                    return Ok(())
                }
                buffer.clear();
                continue;
            }

            let bytecode = match self.assemble_from(&mut asm, &buffer) {
                Err(_) => {
                    buffer.clear();
                    continue;
                }
                Ok(bytecode) => bytecode
            };
            self.vm.add_program(bytecode);
            self.command_buffer.push(buffer.drain(..).collect());
        }
    }

    fn execute_command(&mut self, cmd: &str) -> Result<(), ShouldExit> {
        let mut out = std::io::stdout();
        let mut print = |s: &str| {
            execute!(out, Clear(ClearType::CurrentLine), Print(s)).unwrap_or_else(|_| {});
        };
        let mut step = || {
            if *self.vm.read_pc() < self.vm.program_len() {
                match self.vm.step() {
                    Ok((_, instruction)) => {
                        print(&format!("{}\n", instruction))
                    }
                    Err(e) => println!("Error: {}", e)
                }
            }
        };
        match cmd.trim() {
            "quit" => {
                print("Bye!");
                return Err(ShouldExit);
            }
            "clear" => {
                execute!(out, Clear(ClearType::All)).unwrap_or_else(|_| {});
                return Ok(())
            }
            "run" => {
                self.vm.run();
                print("Done!\n");
                self.execute_command("state")
            }
            "step" => {
                step();
                return Ok(())
            }
            "" => {
                step();
                return Ok(())
            }
            "reset" => {
                self.vm.reset();
                self.command_buffer.clear();
                print("Reset complete.");
                return Ok(())
            }
            "open" => {
                print("Please enter the path to the file you wish to load: ");
                let mut tmp = String::new();
                if let Err(e) = std::io::stdin().read_line(&mut tmp) {
                    print(&format!("Failed to read input: {}", e));
                }
                let filename = Path::new(tmp.trim());
                queue!(std::io::stdout(), MoveToPreviousLine(1)).unwrap_or_else(|_| {});
                let mut f = match File::open(Path::new(&filename)) {
                    Ok(f) => f,
                    Err(e) => {
                        print(&format!("Unable to open file: {}\n", e));
                        return Ok(())
                    }
                };
                let mut contents = String::new();
                if let Err(e) = f.read_to_string(&mut contents) {
                    print(&format!("Unable to read file: {}\n", e));
                    return Ok(())
                };
                let mut asm = Assembler::new();
                let bytecode = match self.assemble_from(&mut asm, &contents) {
                    Err(errs) => {
                        let mut buff = String::new();
                        buff.push_str(&format!("Failed to assemble {}\nErrors found:\n", filename.file_name().unwrap().to_str().unwrap()));
                        for err in errs {
                            buff.push_str(&format!("- {}\n", err));
                        }
                        let msg = "Program could not be loaded.";
                        buff.push_str(&msg);
                        let x = crossterm::terminal::size().unwrap_or_else(|_| (0, 0)).0;
                        let len = msg.len() as u16;
                        if x > len {
                            for _ in 0..x-len {
                                buff.push('-');
                            }
                        }
                        print(&(buff + "\n"));
                        return Ok(())
                    }
                    Ok(bytecode) => bytecode
                };
                print(&format!("running {}:\n", filename.file_name().unwrap().to_str().unwrap()));
                self.vm.add_program(bytecode);
                self.command_buffer.push(contents.drain(..).collect());
                Ok(())
            }
            "state" => {
                print(&format!("{}", self.vm));
                return Ok(())
            }
            "bytecode" => {
                let mut bytecode_str = "Bytecode:\n".to_owned();
                let mut buffer = String::new();
                bytecode_str.push_str("Bytecode:\n");
                for instruction in self.vm.read_program() {
                    buffer.push_str(&format!("{:04x} ", instruction));
                    if buffer.len() >= 16 {
                        bytecode_str.push_str(&format!("{}\n", buffer));
                        buffer.clear();
                    }
                }
                bytecode_str.push_str("End of Bytecode\n");
                print(&bytecode_str);
                return Ok(())
            }
            "reg" => {
                println!("Register File: ");
                let mut buffer = String::from("[ ");
                for (i, register) in self.vm.read_registers().iter().enumerate() {
                    buffer.push_str(&format!("{register}"));
                    if i < 31 {
                        buffer.push_str(", ");
                    }
                }
                buffer.push_str(" ]\nEnd of Register File\n");
                print(&format!("{}", buffer));
                return Ok(())
            }
            "program" => {
                println!("Program:");
                /*let mut instruction: Vec<String> = Vec::new();
                //for byte in self.vm.read_program().iter() {
                for byte in self.vm.read_program().iter().map(|s| s.to_string()) {
                    if instruction.len() == 4 {
                        println!(
                            "{} {:04} {:04} {:04}",
                            OpCode::from_string(&instruction[0]),
                            instruction[1],
                            instruction[2],
                            instruction[3]
                        );
                        instruction.clear();
                    }
                    instruction.push(byte); // *byte
                }*/
                print(&format!("{}{}\n{}", "Program\n", self.command_buffer.join("\n"), "End of Program\n"));
                return Ok(())
            }
            _ => {
                print("Invalid command.\n");
                return Ok(())
            }
        }
    }
}
