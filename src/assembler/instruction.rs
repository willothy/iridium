use super::parser::Token;


#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    pub opcode: Token,
    pub operands: [Option<Token>; 3],
}

impl AssemblerInstruction {
    pub fn new(opcode: Token, operands: [Option<Token>; 3]) -> Self {
        Self { opcode, operands }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut results = vec![];
        match self.opcode {
            Token::Op { code } => {
                results.push(code as u8);
            }
            _ => {
                panic!("Invalid opcode");
            }
        }

        for operand in &self.operands {
            if let Some(token) = operand {
                AssemblerInstruction::extract_operand(token, &mut results)
            }
        }
        results
    }

    fn extract_operand(t: &Token, results: &mut Vec<u8>) {
        match t {
            Token::Register { id } => {
                results.push(*id);
            }
            Token::IntegerOperand { value, sign_bit } => {
                let converted = *value as i16;
                let mut byte1 = converted;
                let mut byte2 = converted >> 8;
                if *sign_bit {
                    byte2 = !byte2;
                    byte1 = (!byte1) + 1;
                }
                results.push(byte2 as u8);
                results.push(byte1 as u8);
            }
            _ => {
                panic!("Opcode found in operand field");
            }
        };
    }
}