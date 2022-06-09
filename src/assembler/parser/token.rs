use crate::opcode::OpCode;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: OpCode },
    Register { id: u8 },
    IntegerOperand { value: u16, sign_bit: bool },
    LabelDeclaration { name: String },
    LabelUsage { name: String },
    Directive { name: String },
}
