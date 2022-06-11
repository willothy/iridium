use crate::opcode::OpCode;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: OpCode },
    Register { id: u8 },
    IntegerOperand { value: u16, sign_bit: bool },
    LabelDeclaration { name: String },
    LabelUsage { name: String },
    Directive { name: String },
    IRString { name: String },
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Op { code } => write!(f, "Op: {}", code),
            Token::Register { id } => write!(f, "Register: {}", id),
            Token::IntegerOperand { value, sign_bit } => {
                if *sign_bit {
                    write!(f, "Int Operand: {}", value)
                } else {
                    write!(f, "Int Operand: {}", value)
                }
            },
            Token::LabelDeclaration { name } => write!(f, "Label Decl: {}", name),
            Token::LabelUsage { name } => write!(f, "Label Usage: {}", name),
            Token::Directive { name } => write!(f, "Directive: {}", name),
            Token::IRString { name } => write!(f, "IRString: {}", name),
        }
    }
}
