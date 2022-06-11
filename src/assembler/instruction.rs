use super::{parser::Token, SymbolTable};
use byteorder::{LittleEndian, WriteBytesExt};

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

    pub fn is_label(&self) -> bool {
        self.label.is_some()
    }

    pub fn is_directive(&self) -> bool {
        self.directive.is_some()
    }

    pub fn is_instruction(&self) -> bool {
        self.opcode.is_some()
    }

    pub fn has_operands(&self) -> bool {
        self.operands.iter().any(|o| o.is_some())
    }

    pub fn directive_name(&self) -> Option<&str> {
        match &self.directive {
            Some(Token::Directive { name }) => Some(name),
            _ => None,
        }
    }

    pub fn get_string_constant(&self) -> Option<String> {
        match &self.operands[0] {
            Some(d) => match d {
                Token::IRString { name } => Some(name.to_string()),
                _ => None,
            },
            None => None,
        }
    }

    pub fn label_name(&self) -> Option<&str> {
        match self.label {
            Some(Token::LabelUsage { ref name }) => Some(name),
            Some(Token::LabelDeclaration { ref name }) => Some(name),
            _ => None,
        }
    }

    pub fn to_bytes(&self, symbols: &SymbolTable) -> Vec<u8> {
        let mut results = vec![];
        /*match &self.opcode {
            Some(Token::Op { code }) => {
                results.push(*code as u8);
            }
            Some(invalid) => {
                panic!("Invalid opcode {}", invalid);
            }
            None => {
                panic!("No opcode");
            }
        }*/
        if let Some(ref token) = self.opcode {
            match token {
                Token::Op { code } => match code {
                    _ => {
                        let b: u8 = (*code).into();
                        results.push(b);
                    }
                },
                _ => {
                    println!("Non-opcode found in opcode field");
                }
            }
        }

        for operand in &self.operands {
            if let Some(token) = operand {
                AssemblerInstruction::extract_operand(token, &mut results, symbols);
            }
        }
        while results.len() < 4 {
            results.push(0);
        }
        results
    }

    fn extract_operand(t: &Token, results: &mut Vec<u8>, symbols: &SymbolTable) {
        match t {
            Token::Register { id } => {
                results.push(*id);
            }
            Token::IntegerOperand { value, sign_bit } => {
                /*let converted = *value;
                let byte1 = converted;
                let byte2 = converted >> 8;
                if *sign_bit {
                    // handle signed numbers
                }
                results.push(byte2 as u8);
                results.push(byte1 as u8);*/
                let val = if *sign_bit {
                    -(*value as i16)
                } else {
                    *value as i16
                };
                let mut wtr = vec![];
                wtr.write_i16::<LittleEndian>(val).unwrap();
                results.push(wtr[1]);
                results.push(wtr[0]);
            }
            /*Token::IntegerOperand { value, sign_bit } => {
                let mut wtr = vec![];
                //wtr.wri
                wtr.write_i16::<LittleEndian>(*value as i16).unwrap();
                results.push(wtr[1]);
                results.push(wtr[0]);
            }*/
            Token::LabelUsage { name } => {
                if let Some(value) = symbols.get_symbol_offset(name) {
                    let mut wtr = vec![];
                    wtr.write_u32::<LittleEndian>(value).unwrap();
                    results.push(wtr[1]);
                    results.push(wtr[0]);
                } else {
                    panic!("No value found for {:?}", name);
                }
            }
            _ => {
                panic!("Opcode found in operand field");
            }
        };
        
    }
}
