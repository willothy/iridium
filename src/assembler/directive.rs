use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{complete::alpha1, streaming::char},
    combinator::opt,
    sequence::tuple,
    IResult,
};

use crate::assembler::instruction::AssemblerInstruction;

use super::parser::{parsers::operand, Token};

pub(in crate::assembler) fn directive_declaration(s: &str) -> IResult<&str, Token, ()> {
    match tuple((char('.'), alpha1))(s) {
        Ok((rem, (_, name))) => Ok((
            rem,
            Token::Directive {
                name: name.to_lowercase(),
            },
        )),
        Err(e) => Err(e),
    }
}

pub(in crate::assembler) fn directive_combined(s: &str) -> IResult<&str, AssemblerInstruction, ()> {
    match tuple((
        char('.'),
        directive_declaration,
        opt(operand),
        opt(operand),
        opt(operand),
    ))(s)
    {
        Ok((rem, (_, directive, operand1, operand2, operand3))) => Ok((
            rem,
            AssemblerInstruction {
                opcode: None,
                operands: [operand1, operand2, operand3],
                directive: Some(directive),
                label: None,
            },
        )),
        Err(e) => Err(e),
    }
}

pub(in crate::assembler) fn directive(s: &str) -> IResult<&str, AssemblerInstruction, ()> {
    match directive_combined(s) {
        Ok((rem, instruction)) => Ok((rem, instruction)),
        Err(e) => Err(e),
    }
}
