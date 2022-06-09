use std::ops::{Index, IndexMut};

use crate::instruction::{Instruction, OpCode, OpCode::*};

#[derive(Debug)]
pub struct VM {
    registers: RegisterSet,
    pc: usize,
    program: Vec<u8>,
    remainder: u32,
    equal_flag: bool,
}

#[derive(Debug)]
pub struct RegisterSet {
    pub registers: [i32; 32],
}

impl RegisterSet {
    pub fn new() -> Self {
        RegisterSet { registers: [0; 32] }
    }

    pub fn get(&self, index: usize) -> Result<&i32, String> {
        if index < self.registers.len() {
            Ok(&self.registers[index])
        } else {
            Err(format!("Register index {} out of bounds", index))
        }
    }

    pub fn set(&mut self, index: usize, value: i32) -> Result<(), String> {
        if index < self.registers.len() {
            self.registers[index] = value;
            Ok(())
        } else {
            Err(format!("Register index {} out of bounds", index))
        }
    }
}

impl Index<usize> for RegisterSet {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.registers[index]
    }
}

impl IndexMut<usize> for RegisterSet {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.registers[index]
    }
}

impl VM {
    pub fn new() -> Self {
        Self {
            registers: RegisterSet::new(),
            program: vec![],
            pc: 0,
            remainder: 0,
            equal_flag: false,
        }
    }

    pub fn read_program(&self) -> &Vec<u8> {
        &self.program
    }

    pub fn read_registers(&self) -> &[i32] {
        &self.registers.registers
    }

    pub fn add_command(&mut self, command: Vec<u8>) {
        self.program.extend(command);
    }

    pub fn run(&mut self) {
        let mut done = false;
        while !done {
            match self.execute_instruction() {
                Ok(is_done) => done = is_done,
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }

    fn execute_instruction(&mut self) -> Result<bool, String> {
        if self.pc >= self.program.len() {
            return Ok(true);
        }
        let instruction = self.get_next_instruction();
        let operands = instruction.operands();
        match instruction.opcode() {
            LOAD => {
                self.registers.set(
                    operands[0] as usize,
                    (Self::conv_u8s_u16(&[operands[1], operands[2]]) as u32) as i32,
                )?;
            }
            JMP => {
                self.pc = *self.registers.get(operands[0] as usize)? as usize;
            }
            JMPF => {
                self.pc += *self.registers.get(operands[0] as usize)? as usize;
            }
            JMPB => {
                self.pc -= *self.registers.get(operands[0] as usize)? as usize;
            }
            ADD => {
                self.registers.set(
                    operands[2] as usize,
                    self.registers.get(operands[0] as usize)?
                        + self.registers.get(operands[1] as usize)?,
                )?;
            }
            SUB => {
                self.registers.set(
                    operands[2] as usize,
                    self.registers.get(operands[0] as usize)?
                        - self.registers.get(operands[1] as usize)?,
                )?;
            }
            MUL => {
                self.registers.set(
                    operands[2] as usize,
                    self.registers.get(operands[0] as usize)?
                        * self.registers.get(operands[1] as usize)?,
                )?;
            }
            DIV => {
                let register1 = *self.registers.get(operands[0] as usize)?;
                let register2 = *self.registers.get(operands[1] as usize)?;
                self.registers
                    .set(operands[2] as usize, register1 / register2)?;
                self.remainder = (register1 % register2) as u32;
            }
            EQ => {
                self.equal_flag = *self.registers.get(operands[0] as usize)?
                    == *self.registers.get(operands[1] as usize)?;
            }
            NE => {
                self.equal_flag = *self.registers.get(operands[0] as usize)?
                    != *self.registers.get(operands[1] as usize)?;
            }
            JEQ => {
                if self.equal_flag {
                    self.pc = *self.registers.get(operands[0] as usize)? as usize;
                }
            }
            JNE => {
                if !self.equal_flag {
                    self.pc = *self.registers.get(operands[0] as usize)? as usize;
                }
            }
            HLT => {
                println!("HLT encountered");
                return Ok(true); // Done
            }
            _ => {
                println!("Unrecognized opcode");
                return Ok(true); // Done
            }
        }
        Ok(false)
    }

    fn get_next_instruction(&mut self) -> Instruction {
        let opcode = self.decode_opcode();
        let operands: Vec<u8> = vec![0 as u8, 0, 0]
            .iter()
            .map(|_| {
                let byte = self.program[self.pc];
                self.pc += 1;
                byte
            })
            .collect();

        Instruction::new(opcode, operands)
    }

    fn decode_opcode(&mut self) -> OpCode {
        let opcode = OpCode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }

    fn conv_u8s_u16(bytes: &[u8]) -> u16 {
        ((bytes[0] as u16) << 8) | (bytes[1] as u16)
    }

    // Test helpers
    pub fn run_once(&mut self) -> Result<(), String> {
        self.execute_instruction()?;
        Ok(())
    }
}

impl std::fmt::Display for VM {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = String::new();
        for (i, reg) in self.read_registers().iter().enumerate() {
            s.push_str(&format!("{}", reg));
            if i < 32 {
                s.push_str(", ");
            }
        }
        for i in 0..self.registers.registers.len() {
            write!(f, "{}", self.registers[i])?;
            if i < 32 {
                write!(f, ", ")?;
            }
        }
        write!(f, " ]\n")?;
        write!(f, "PC: {}\n", self.pc)?;
        write!(f, "Remainder: {}\n", self.remainder)?;
        write!(f, "Equal flag: {}\n", self.equal_flag)?;
        write!(f, "Program: {}\n", s)?;
        Ok(())
    }
}

/// Tests for vm
#[cfg(test)]
mod tests {
    use super::*;

    trait VMTestHelpers {
        fn with_program(self, program: Vec<u8>) -> Self;
        fn run_for(&mut self, n: usize);
        fn reset(&mut self);
        fn next_8_bits(&mut self) -> u8;
        fn next_16_bits(&mut self) -> u16;
    }

    impl VMTestHelpers for VM {
        fn with_program(mut self, program: Vec<u8>) -> Self {
            self.program = program;
            self
        }

        fn run_for(&mut self, n: usize) {
            for _ in 0..n {
                self.run_once();
            }
        }

        fn reset(&mut self) {
            self.pc = 0;
            self.registers = RegisterSet::new();
            self.remainder = 0;
            self.program = vec![];
            self.equal_flag = false;
        }

        fn next_8_bits(&mut self) -> u8 {
            let byte = self.program[self.pc];
            self.pc += 1;
            byte
        }

        fn next_16_bits(&mut self) -> u16 {
            // Shift value of first byte left by 8 bits, so last 8 bits are 00000000
            // Logical or the second byte (as a u16 with first 8 bytes 00000000), creating a u16 from both bytes.
            let result = ((self.program[self.pc] as u16) << 8) | (self.program[self.pc + 1] as u16);
            self.pc += 2;
            result
        }
    }

    fn expand(num: u16) -> [u8; 2] {
        [(num >> 8) as u8, num as u8]
    }

    fn create_load_unchecked(vm: &mut VM, register: u8, number: [u8; 2]) {
        vm.program.extend(vec![LOAD as u8, register]);
        vm.program.extend(number);
    }

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }

    #[test]
    fn test_hlt() {
        let mut test_vm = VM::new().with_program(vec![HLT as u8, 0, 0, 0]);
        test_vm.run();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_igl() {
        let mut test_vm = VM::new().with_program(vec![IGL as u8, 0, 0, 0]);
        test_vm.run();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_load() {
        let mut program: Vec<u8> = vec![LOAD as u8, 0];
        program.extend(expand(500));

        let mut test_vm = VM::new().with_program(program);
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_add() {
        let mut test_vm = VM::new();
        create_load_unchecked(&mut test_vm, 0, expand(10));
        create_load_unchecked(&mut test_vm, 1, expand(15));

        test_vm.program.extend(vec![ADD as u8, 0, 1, 2]);
        test_vm.run();
        assert_eq!(test_vm.registers[2], 25);
    }

    #[test]
    fn test_sub() {
        let mut test_vm = VM::new();
        create_load_unchecked(&mut test_vm, 0, expand(10));
        create_load_unchecked(&mut test_vm, 1, expand(15));

        test_vm.program.extend(vec![SUB as u8, 1, 0, 2]);
        test_vm.run();
        assert_eq!(test_vm.registers[2], 5);
    }

    #[test]
    fn test_mul() {
        let mut test_vm = VM::new();
        create_load_unchecked(&mut test_vm, 0, expand(2));
        create_load_unchecked(&mut test_vm, 1, expand(15));

        test_vm.program.extend(vec![MUL as u8, 0, 1, 2]);
        test_vm.run();
        assert_eq!(test_vm.registers[2], 30);
    }

    #[test]
    fn test_div() {
        let mut test_vm = VM::new();
        create_load_unchecked(&mut test_vm, 0, expand(30));
        create_load_unchecked(&mut test_vm, 1, expand(15));

        test_vm.program.extend(vec![DIV as u8, 0, 1, 2]);

        test_vm.run();
        assert_eq!(test_vm.registers[2], 2);
    }

    #[test]
    fn test_div_remainder() {
        let mut test_vm = VM::new();
        create_load_unchecked(&mut test_vm, 0, expand(6));
        create_load_unchecked(&mut test_vm, 1, expand(4));

        test_vm.program.extend(vec![DIV as u8, 0, 1, 2]);

        test_vm.run();
        assert_eq!(test_vm.remainder, 2);
    }

    #[test]
    fn test_jmp() {
        let mut test_vm = VM::new();
        create_load_unchecked(&mut test_vm, 1, [0, 0]); // load register 1 with 0
        test_vm.program.extend(vec![JMP as u8, 1, 0, 0]); // Jump to the location in register 1 (the beginning of the program, infinite loop)
        test_vm.run_for(2); // Execute the load and jump, don't allow infinite loop
        assert_eq!(test_vm.pc, 0);
    }

    #[test]
    fn test_jmpf() {
        let mut test_vm = VM::new();
        create_load_unchecked(&mut test_vm, 0, expand(4)); // Load 8 into register 0
        test_vm.program.extend(vec![JMPF as u8, 0, 0, 0]); // Jump 8 bytes forwards
        create_load_unchecked(&mut test_vm, 0, expand(15)); // Load 15 into register 0 (should be skipped)

        test_vm.run_for(3);
        assert_eq!(test_vm.pc, 12);
        assert_eq!(test_vm.registers[0], 4);
    }

    #[test]
    fn test_jmpb() {
        let mut test_vm = VM::new();
        test_vm.pc = 4; // skip first load
        create_load_unchecked(&mut test_vm, 0, expand(0)); // Load 0 into register 0
        create_load_unchecked(&mut test_vm, 0, expand(12)); // Load 12 into register 0
        test_vm.program.extend(vec![JMPB as u8, 0, 0, 0]); // Jump 8 bytes back (to first load)

        test_vm.run_for(3);
        assert_eq!(test_vm.registers[0], 0); // Ensure first load executed after jump
    }

    #[test]
    fn test_eq_true() {
        let mut test_vm = VM::new();
        create_load_unchecked(&mut test_vm, 0, expand(10));
        create_load_unchecked(&mut test_vm, 1, expand(10));
        test_vm.program.extend(vec![EQ as u8, 0, 1, 0]);
        test_vm.run_for(3);
        assert_eq!(test_vm.equal_flag, true);
    }

    #[test]
    fn test_eq_false() {
        let mut test_vm = VM::new();
        create_load_unchecked(&mut test_vm, 0, expand(20));
        create_load_unchecked(&mut test_vm, 1, expand(10));
        test_vm.program.extend(vec![EQ as u8, 0, 1, 0]);
        test_vm.run();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_jeq() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 7;
        test_vm.equal_flag = true;
        test_vm.program = vec![JEQ as u8, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 7);
    }

    #[test]
    fn test_jne() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 7;
        test_vm.equal_flag = false;
        test_vm.program = vec![JNE as u8, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 7);
    }
}
