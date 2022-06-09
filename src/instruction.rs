

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
    NE,
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
    IGL = 100,
}

impl From<OpCode> for String {
    fn from(op: OpCode) -> Self {
        use self::OpCode::*;
        match op {
            LOAD => "LOAD".to_string(),
            ADD => "ADD".to_string(),
            SUB => "SUB".to_string(),
            MUL => "MUL".to_string(),
            DIV => "DIV".to_string(),
            HLT => "HLT".to_string(),
            JMP => "JMP".to_string(),
            JMPF => "JMPF".to_string(),
            JMPB => "JMPB".to_string(),
            EQ => "EQ".to_string(),
            NE => "NE".to_string(),
            GTE => "GTE".to_string(),
            LTE => "LTE".to_string(),
            LT => "LT".to_string(),
            GT => "GT".to_string(),
            JEQ => "JEQ".to_string(),
            NOP => "NOP".to_string(),
            ALOC => "ALOC".to_string(),
            INC => "INC".to_string(),
            DEC => "DEC".to_string(),
            DJMPE => "DJMPE".to_string(),
            IGL => "IGL".to_string(),
            PRTS => "PRTS".to_string(),
            LOADF64 => "LOADF64".to_string(),
            ADDF64 => "ADDF64".to_string(),
            SUBF64 => "SUBF64".to_string(),
            MULF64 => "MULF64".to_string(),
            DIVF64 => "DIVF64".to_string(),
            EQF64 => "EQF64".to_string(),
            NEQF64 => "NEQF64".to_string(),
            GTF64 => "GTF64".to_string(),
            GTEF64 => "GTEF64".to_string(),
            LTF64 => "LTF64".to_string(),
            LTEF64 => "LTEF64".to_string(),
            SHL => "SHL".to_string(),
            SHR => "SHR".to_string(),
            AND => "AND".to_string(),
            OR => "OR".to_string(),
            XOR => "XOR".to_string(),
            NOT => "NOT".to_string(),
            LUI => "LUI".to_string(),
            CLOOP => "CLOOP".to_string(),
            LOOP => "LOOP".to_string(),
            LOADM => "LOADM".to_string(),
            SETM => "SETM".to_string(),
            PUSH => "PUSH".to_string(),
            POP => "POP".to_string(),
            CALL => "CALL".to_string(),
            RET => "RET".to_string(),
            JNE => "JNE".to_string(),
        }
    }
}

impl From<OpCode> for u8 {
    fn from(op: OpCode) -> Self {
        op as u8
    }
}

/// We implement this trait to make it easy to convert from a u8 to an Opcode
impl From<u8> for OpCode {
    fn from(v: u8) -> Self {
        use self::OpCode::*;
        match v {
            0 => LOAD,
            1 => ADD,
            2 => SUB,
            3 => MUL,
            4 => DIV,
            5 => HLT,
            6 => JMP,
            7 => JMPF,
            8 => JMPB,
            9 => EQ,
            10 => NE,
            11 => GTE,
            12 => LTE,
            13 => LT,
            14 => GT,
            15 => JEQ,
            16 => NOP,
            17 => ALOC,
            18 => INC,
            19 => DEC,
            20 => DJMPE,
            21 => PRTS,
            22 => LOADF64,
            23 => ADDF64,
            24 => SUBF64,
            25 => MULF64,
            26 => DIVF64,
            27 => EQF64,
            28 => NEQF64,
            29 => GTF64,
            30 => GTEF64,
            31 => LTF64,
            32 => LTEF64,
            33 => SHL,
            34 => SHR,
            35 => AND,
            36 => OR,
            37 => XOR,
            38 => NOT,
            39 => LUI,
            40 => CLOOP,
            41 => LOOP,
            42 => LOADM,
            43 => SETM,
            44 => PUSH,
            45 => POP,
            46 => CALL,
            47 => RET,
            48 => JNE,
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
mod tests {
    use super::*;

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(OpCode::HLT);
        assert_eq!(instruction.opcode, OpCode::HLT);
    }
}
