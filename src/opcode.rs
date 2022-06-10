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

impl std::fmt::Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(*self))
    }
}

impl OpCode {
    pub fn padded(self) -> String {
        let mut padded: String = self.into();
        while padded.len() < 4 {
            padded.push(' ');
        }
        padded
    }
}

impl From<&str> for OpCode {
    fn from(op_str: &str) -> Self {
        match op_str {
            "load" => OpCode::LOAD,
            "add" => OpCode::ADD,
            "sub" => OpCode::SUB,
            "mul" => OpCode::MUL,
            "div" => OpCode::DIV,
            "hlt" => OpCode::HLT,
            "jmp" => OpCode::JMP,
            "jmpf" => OpCode::JMPF,
            "jmpb" => OpCode::JMPB,
            "eq" => OpCode::EQ,
            "neq" => OpCode::NE,
            "gte" => OpCode::GTE,
            "gt" => OpCode::GT,
            "lte" => OpCode::LTE,
            "lt" => OpCode::LT,
            "jeq" => OpCode::JEQ,
            "nop" => OpCode::NOP,
            "aloc" => OpCode::ALOC,
            "inc" => OpCode::INC,
            "dec" => OpCode::DEC,
            "djmpe" => OpCode::DJMPE,
            "prts" => OpCode::PRTS,
            "loadf64" => OpCode::LOADF64,
            "addf64" => OpCode::ADDF64,
            "subf64" => OpCode::SUBF64,
            "mulf64" => OpCode::MULF64,
            "divf64" => OpCode::DIVF64,
            "eqf64" => OpCode::EQF64,
            "neqf64" => OpCode::NEQF64,
            "gtf64" => OpCode::GTF64,
            "gtef64" => OpCode::GTEF64,
            "ltf64" => OpCode::LTF64,
            "ltef64" => OpCode::LTEF64,
            "shl" => OpCode::SHL,
            "shr" => OpCode::SHR,
            "and" => OpCode::AND,
            "or" => OpCode::OR,
            "xor" => OpCode::XOR,
            "not" => OpCode::NOT,
            "lui" => OpCode::LUI,
            "cloop" => OpCode::CLOOP,
            "loop" => OpCode::LOOP,
            "loadm" => OpCode::LOADM,
            "setm" => OpCode::SETM,
            "push" => OpCode::PUSH,
            "pop" => OpCode::POP,
            "call" => OpCode::CALL,
            "ret" => OpCode::RET,
            "jne" => OpCode::JNE,
            _ => OpCode::IGL,
        }
    }
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

/// We implement this trait to make it easy to convert from a u8 to an OpCode
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
