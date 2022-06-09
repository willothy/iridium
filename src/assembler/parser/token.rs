use crate::instruction::OpCode;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: OpCode },
    Register { id: u8 },
    IntegerOperand { value: u16, sign_bit: bool },
}