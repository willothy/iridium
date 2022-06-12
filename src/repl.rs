use std::{
    fs::File,
    io::{self, Read, Stdout},
    path::Path,
};

use crate::{
    assembler::{Assembler, AssemblerError},
    vm::VM,
};

use crossterm::{
    cursor::{position, MoveTo, MoveToPreviousLine},
    execute, queue,
    style::Print,
    terminal::{Clear, ClearType},
};

struct ShouldExit;

pub struct REPL {
    command_buffer: Vec<String>,
    assembler: Assembler,
    vm: VM,
}

impl REPL {
    pub fn new() -> Self {
        Self {
            command_buffer: Vec::new(),
            assembler: Assembler::new(),
            vm: VM::new(),
        }
    }

    fn assemble_from<'asm>(
        &'asm mut self,
        program: &'asm str,
    ) -> Result<&'asm mut Vec<u8>, &'asm Vec<AssemblerError>> {
        match self.assembler.assemble(program) {
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
        execute!(output, crossterm::cursor::EnableBlinking).unwrap_or_else(|_| {});
        loop {
            execute!(
                output,
                MoveTo(0, position().unwrap().1),
                Clear(ClearType::CurrentLine),
                Print(">>> ")
            )
            .unwrap_or_else(|_| {});
            input.read_line(&mut buffer).expect("Failed to read stdin");
            execute!(output, MoveToPreviousLine(1)).unwrap_or_else(|_| {});
            if buffer.starts_with("!") || buffer.starts_with("\n") {
                if let Err(ShouldExit) = self.execute_command(&buffer[1..]) {
                    return Ok(());
                }
                buffer.clear();
                continue;
            }

            let insert = buffer.starts_with("-") && {
                buffer = buffer[1..].to_string();
                true
            };

            if let Err(errs) = self.assembler.assemble(&("    ".to_owned() + &buffer + "\n")) {
                Self::log_errors(&mut output, &errs, &buffer);
                continue
            }

            // Insert or append to program
            if insert {
                self.vm.insert_into_program(&mut self.assembler.last_instruction(), *self.vm.read_pc());
            } else {
                self.vm.add_program(&mut self.assembler.last_instruction());
            }
            let cmd = buffer.drain(..).collect::<String>();
            self.command_buffer.push("    ".to_string() + &cmd);
        }
    }

    fn log_errors(out: &mut Stdout, errs: &Vec<AssemblerError>, name: &str) {
        let name = name.trim_end_matches("\n");
        let mut buff = String::new();
        buff.push_str(&format!(
            "Failed to assemble '{}'\nErrors found:\n",
            &name
        ));
        for err in errs {
            buff.push_str(&format!("- {}\n", err));
        }
        let msg = format!("Failed to assemble '{}' ", &name);//"Program could not be loaded ";
        buff.push_str(&msg);
        let x = crossterm::terminal::size().unwrap_or_else(|_| (0, 0)).0;
        let len = msg.len() as u16;
        if x > len {
            for _ in 0..x - len {
                buff.push('-');
            }
        }
        Self::print(out, &(buff + "\n"));
    }

    pub fn print(out: &mut Stdout, s: &str) {
        execute!(out, Clear(ClearType::CurrentLine), Print(s)).unwrap_or_else(|_| {});
    }

    fn execute_command(&mut self, cmd: &str) -> Result<(), ShouldExit> {
        let mut out = std::io::stdout();
        let mut print = |s: &str| {
            Self::print(&mut out, s);
        };
        let mut step = || {
            if *self.vm.read_pc() < self.vm.program_len() {
                match self.vm.step() {
                    Ok((_, instruction)) => print(&format!("{}\n", instruction)),
                    Err(e) => println!("Error: {}", e),
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
                return Ok(());
            }
            "run" => {
                self.vm.run();
                print("Done!\n");
                self.execute_command("state")
            }
            "step" => {
                step();
                return Ok(());
            }
            "" => {
                step();
                return Ok(());
            }
            "reset" => {
                self.vm.reset();
                self.command_buffer.clear();
                print("Reset complete.");
                return Ok(());
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
                        return Ok(());
                    }
                };
                let mut contents = String::new();
                if let Err(e) = f.read_to_string(&mut contents) {
                    print(&format!("Unable to read file: {}\n", e));
                    return Ok(());
                };
                /* let bytecode = match self.assemble_from(&contents) {
                    Err(errs) => {
                        Self::log_errors(&mut out, errs, filename.to_str().unwrap_or(""));
                        return Ok(());
                    }
                    Ok(bytecode) => bytecode,
                }; */
                let bytecode = match self.assembler.assemble(&contents) {
                    Ok(bytecode) => bytecode,
                    Err(e) => {
                        Self::log_errors(&mut out, &e, filename.to_str().unwrap_or(""));
                        return Ok(());
                    }
                };
                print(&format!(
                    "running {}:\n",
                    filename.file_name().unwrap().to_str().unwrap()
                ));
                self.vm.add_program(bytecode);
                self.command_buffer.push(contents.drain(..).collect());
                Ok(())
            }
            "state" => {
                print(&format!("{}", self.vm));
                return Ok(());
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
                return Ok(());
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
                return Ok(());
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
                print(&format!(
                    "{}{}\n{}",
                    "Program\n",
                    self.command_buffer.join("\n"),
                    "End of Program\n"
                ));
                return Ok(());
            }
            _ => {
                print("Invalid command.\n");
                return Ok(());
            }
        }
    }
}
