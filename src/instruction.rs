/// Represents an opcode, which tells our interpreter what to do with the following operands
#[derive(Copy, Clone, Debug, PartialEq)]
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
    JMPE,
    NOP,
    ALOC,
    INC,
    DEC,
    DJMPE,
    IGL,
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
}

impl From<OpCode> for u8 {
    fn from(op: OpCode) -> Self {
        match op {
            OpCode::LOAD => 0,
            OpCode::ADD => 1,
            OpCode::SUB => 2,
            OpCode::MUL => 3,
            OpCode::DIV => 4,
            OpCode::HLT => 5,
            OpCode::JMP => 6,
            OpCode::JMPF => 7,
            OpCode::JMPB => 8,
            OpCode::EQ => 9,
            OpCode::NEQ => 10,
            OpCode::GTE => 11,
            OpCode::LTE => 12,
            OpCode::LT => 13,
            OpCode::GT => 14,
            OpCode::JMPE => 15,
            OpCode::NOP => 16,
            OpCode::ALOC => 17,
            OpCode::INC => 18,
            OpCode::DEC => 19,
            OpCode::DJMPE => 20,
            OpCode::PRTS => 21,
            OpCode::LOADF64 => 22,
            OpCode::ADDF64 => 23,
            OpCode::SUBF64 => 24,
            OpCode::MULF64 => 25,
            OpCode::DIVF64 => 26,
            OpCode::EQF64 => 27,
            OpCode::NEQF64 => 28,
            OpCode::GTF64 => 29,
            OpCode::GTEF64 => 30,
            OpCode::LTF64 => 31,
            OpCode::LTEF64 => 32,
            OpCode::SHL => 33,
            OpCode::SHR => 34,
            OpCode::AND => 35,
            OpCode::OR => 36,
            OpCode::XOR => 37,
            OpCode::NOT => 38,
            OpCode::LUI => 39,
            OpCode::CLOOP => 40,
            OpCode::LOOP => 41,
            OpCode::LOADM => 42,
            OpCode::SETM => 43,
            OpCode::PUSH => 44,
            OpCode::POP => 45,
            OpCode::CALL => 46,
            OpCode::RET => 47,
            OpCode::IGL => 100,
        }
    }
}
/// We implement this trait to make it easy to convert from a u8 to an Opcode
impl From<u8> for OpCode {
    fn from(v: u8) -> Self {
        match v {
            0 => OpCode::LOAD,
            1 => OpCode::ADD,
            2 => OpCode::SUB,
            3 => OpCode::MUL,
            4 => OpCode::DIV,
            5 => OpCode::HLT,
            6 => OpCode::JMP,
            7 => OpCode::JMPF,
            8 => OpCode::JMPB,
            9 => OpCode::EQ,
            10 => OpCode::NEQ,
            11 => OpCode::GTE,
            12 => OpCode::LTE,
            13 => OpCode::LT,
            14 => OpCode::GT,
            15 => OpCode::JMPE,
            16 => OpCode::NOP,
            17 => OpCode::ALOC,
            18 => OpCode::INC,
            19 => OpCode::DEC,
            20 => OpCode::DJMPE,
            21 => OpCode::PRTS,
            22 => OpCode::LOADF64,
            23 => OpCode::ADDF64,
            24 => OpCode::SUBF64,
            25 => OpCode::MULF64,
            26 => OpCode::DIVF64,
            27 => OpCode::EQF64,
            28 => OpCode::NEQF64,
            29 => OpCode::GTF64,
            30 => OpCode::GTEF64,
            31 => OpCode::LTF64,
            32 => OpCode::LTEF64,
            33 => OpCode::SHL,
            34 => OpCode::SHR,
            35 => OpCode::AND,
            36 => OpCode::OR,
            37 => OpCode::XOR,
            38 => OpCode::NOT,
            39 => OpCode::LUI,
            40 => OpCode::CLOOP,
            41 => OpCode::LOOP,
            42 => OpCode::LOADM,
            43 => OpCode::SETM,
            44 => OpCode::PUSH,
            45 => OpCode::POP,
            46 => OpCode::CALL,
            47 => OpCode::RET,
            _ => OpCode::IGL,
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
mod tests {
    use super::*;

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(OpCode::HLT);
        assert_eq!(instruction.opcode, OpCode::HLT);
    }
}
