use crate::assembler::instruction::AssemblerInstruction;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub instructions: Vec<AssemblerInstruction>,
}

impl Program {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut results = vec![];
        for instruction in &self.instructions {
            results.append(&mut instruction.to_bytes());
        }
        results
    }
}

pub fn parse_program(s: &str) -> Result<Program, nom::Err<()>> {
    Ok(super::parsers::program(s)?.1)
}