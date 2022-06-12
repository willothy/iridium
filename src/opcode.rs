use std::str::FromStr;

use num_traits::FromPrimitive;
use strum::{Display, EnumString};

enum_from_primitive! {
    /// Represents an opcode, which tells our interpreter what to do with the following operands
    #[derive(Copy, Clone, Debug, PartialEq, Display, EnumString)]
    #[strum(ascii_case_insensitive)]
    pub enum OpCode {
        LOAD,
        ADD,
        SUB,
        MUL,
        DIV,
        HLT,
        JMP,
        JMPF,
        JMPB,
        EQ,
        NEQ,
        GTE,
        LTE,
        LT,
        GT,
        JEQ,
        NOP,
        ALOC,
        INC,
        DEC,
        DJMPE,
        PRTS,
        LOADF64,
        ADDF64,
        SUBF64,
        MULF64,
        DIVF64,
        EQF64,
        NEQF64,
        GTF64,
        GTEF64,
        LTF64,
        LTEF64,
        SHL,
        SHR,
        AND,
        OR,
        XOR,
        NOT,
        LUI,
        CLOOP,
        LOOP,
        LOADM,
        SETM,
        PUSH,
        POP,
        CALL,
        RET,
        JNE,
        JMPL,
        IGL = 100,
    }
}

impl OpCode {
    pub fn padded(self) -> String {
        let mut padded: String = self.to_string();
        while padded.len() < 4 {
            padded.push(' ');
        }
        padded
    }

    pub fn from_string(s: &str) -> Self {
        match OpCode::from_str(s) {
            Ok(opcode) => opcode,
            Err(_) => OpCode::IGL,
        }
    }
}

impl From<OpCode> for u8 {
    fn from(op: OpCode) -> Self {
        op as u8
    }
}

impl From<u8> for OpCode {
    fn from(v: u8) -> Self {
        use self::OpCode::*;
        match OpCode::from_u8(v) {
            Some(op) => op,
            None => IGL,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Instruction {
    opcode: OpCode,
    operands: Vec<u8>,
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} ", self.opcode)?;
        for op in &self.operands {
            write!(f, "{:02} ", op)?;
        }
        Ok(())
    }
}

pub trait Tou8Vec {
    fn to_u8_vec(&self) -> Vec<u8>;
}

impl Tou8Vec for Instruction {
    fn to_u8_vec(&self) -> Vec<u8> {
        let mut v = Vec::new();
        v.push(self.opcode.into());
        v.extend(self.operands.iter());
        v
    }
}

impl Into<Vec<u8>> for Instruction {
    fn into(self) -> Vec<u8> {
        let mut bytes = vec![self.opcode.into()];
        bytes.extend(self.operands);
        bytes
    }
}

impl Instruction {
    pub fn new(opcode: OpCode, operands: Vec<u8>) -> Instruction {
        Instruction { opcode, operands }
    }

    pub fn opcode(&self) -> &OpCode {
        &self.opcode
    }

    pub fn operands(&self) -> &Vec<u8> {
        &self.operands
    }
}

/// Tests for instruction
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(OpCode::HLT, vec![]);
        assert_eq!(instruction.opcode, OpCode::HLT);
    }
}
