use super::parser::Token;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    pub opcode: Option<Token>,
    pub operands: [Option<Token>; 3],
    pub label: Option<Token>,
    pub directive: Option<Token>,
}

impl AssemblerInstruction {
    pub fn new(
        opcode: Option<Token>,
        operands: [Option<Token>; 3],
        label: Option<Token>,
        directive: Option<Token>,
    ) -> Self {
        Self {
            opcode,
            operands,
            label,
            directive,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut results = vec![];
        match self.opcode {
            Some(Token::Op { code }) => {
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
        while results.len() < 4 {
            results.push(0);
        }
        results
    }

    fn extract_operand(t: &Token, results: &mut Vec<u8>) {
        match t {
            Token::Register { id } => {
                results.push(*id);
            }
            Token::IntegerOperand { value, sign_bit } => {
                let converted = *value;
                let byte1 = converted;
                let byte2 = converted >> 8;
                if *sign_bit {
                    // handle signed numbers
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
