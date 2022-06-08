#[derive(Debug, PartialEq)]
pub enum OpCode {
    HLT,
    IGL,
}

impl From<u8> for OpCode {
    fn from(opcode: u8) -> Self {
        use self::OpCode::*;
        match opcode {
            0x0 => HLT,
            _ => IGL,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: OpCode,
}

impl Instruction {
    pub fn new(opcode: OpCode) -> Instruction {
        Instruction { opcode }
    }
}

/// Tests for instruction
#[cfg(test)]
mod instruction_tests {
    use super::*;

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(OpCode::HLT);
        assert_eq!(instruction.opcode, OpCode::HLT);
    }
}
