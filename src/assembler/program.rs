use crate::assembler::instruction::AssemblerInstruction;

use super::SymbolTable;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub instructions: Vec<AssemblerInstruction>,
}

impl Program {
    pub fn to_bytes(&self, symbols: &SymbolTable) -> Vec<u8> {
        let mut results = vec![];
        for instruction in &self.instructions {
            results.append(&mut instruction.to_bytes(symbols));
        }
        results
    }
}

pub fn parse_program(s: &str) -> Result<super::Program, nom::Err<()>> {
    Ok(super::parser::parsers::program(s)?.1)
}
