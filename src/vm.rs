use crate::instruction::{OpCode, OpCode::*};

pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
    remainder: u32,
    equal_flag: bool,
}

impl VM {
    pub fn new() -> Self {
        Self {
            registers: [0; 32],
            program: vec![],
            pc: 0,
            remainder: 0,
            equal_flag: false
        }
    }

    pub fn with_program(mut self, program: Vec<u8>) -> Self {
        self.program = program;
        self
    }

    pub fn run(&mut self) {
        let mut done = false;
        while !done {
            done = self.execute_instruction();
        }
    }

    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return true;
        }
        match self.decode_opcode() {
            LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u32;
                self.registers[register] = number as i32;
                false // Continue
            },
            JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;
                false // Continue
            },
            JMPF => {
                self.pc += self.registers[self.next_8_bits() as usize] as usize;
                false
            },
            JMPB => {
                self.pc -= self.registers[self.next_8_bits() as usize] as usize;
                false
            },
            ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 + register2;
                false // Continue
            },
            SUB => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 - register2;
                false // Continue
            },
            MUL => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 * register2;
                false // Continue
            },
            DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as u32;
                false // Continue
            },
            EQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register1 == register2;
                self.next_8_bits();
                false // Continue
            },
            HLT => {
                println!("HLT encountered");
                true // Done
            },
            _ => {
                println!("Unrecognized opcode");
                true // Done
            }
        }
    }

    fn decode_opcode(&mut self) -> OpCode {
        let opcode = OpCode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
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

    // Test helpers
    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    pub fn run_for(&mut self, n: usize) {
        for _ in 0..n {
            self.run_once();
        }
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.registers = [0; 32];
        self.remainder = 0;
        self.program = vec![];
    }
}

/// Tests for vm
#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_igl() {
        let mut test_vm = VM::new().with_program(vec![IGL as u8, 0, 0, 0]);
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
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
        create_load_unchecked(&mut test_vm, 0, [0, 6]); // Load 6 in
        test_vm.program.extend(vec![JMPF as u8, 0, 0, 0]); // Jump 6 bytes (past the last 2 bytes of this command, and the next command)
        create_load_unchecked(&mut test_vm, 2, [0, 15]); //
        test_vm.program.extend(vec![ADD as u8, 2, 0, 3]);
        test_vm.run();
        assert_eq!(test_vm.registers[3], 6);
    }

    #[test]
    fn test_jmpb() {
        let mut test_vm = VM::new();
        test_vm.pc = 4;
        create_load_unchecked(&mut test_vm, 0, expand(0)); // Load 8 into register 0
        create_load_unchecked(&mut test_vm, 0, expand(8)); // Load 8 into register 0
        test_vm.program.extend(vec![JMPB as u8, 0, 0, 0]); // Jump 8 bytes back

        test_vm.run_for(4);
        assert_eq!(test_vm.registers[0], 0);
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
}
