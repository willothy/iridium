use super::{formats::*, Token};
use crate::assembler::Program;
use crate::opcode::OpCode;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::one_of,
    character::complete::{alpha1, char, digit1},
    combinator::{map_res, recognize},
    multi::{many0, many1},
    sequence::{terminated, tuple},
    IResult,
};

pub fn program(s: &str) -> IResult<&str, Program, ()> {
    match many1(alt((
        op,
        op_reg,
        op_reg_reg,
        op_reg_reg_reg,
        op_reg_val,
        instruction_combined,
    )))(s)
    {
        Ok((rem, instructions)) => Ok((rem, Program { instructions })),
        Err(e) => Err(e),
    }
}

pub fn operand(s: &str) -> IResult<&str, Token, ()> {
    match alt((register, integer_operand))(s) {
        Ok((rem, token)) => Ok((rem, token)),
        Err(e) => Err(e),
    }
}

pub fn opcode(s: &str) -> IResult<&str, Token, ()> {
    match alpha1(s) {
        Ok((rem, opcode)) => Ok((
            rem,
            Token::Op {
                code: match OpCode::from(&opcode.to_lowercase()[..]) {
                    OpCode::IGL => return Err(nom::Err::Error(())),
                    opcode => opcode,
                },
            },
        )),
        Err(e) => Err(e),
    }
}

pub fn register(s: &str) -> IResult<&str, Token, ()> {
    match tuple((char('$'), digit1))(s) {
        Ok((rem, (_, number))) => Ok((
            rem,
            Token::Register {
                id: number.parse().unwrap(),
            },
        )),
        Err(e) => Err(e),
    }
}

pub fn integer_operand(mut s: &str) -> IResult<&str, Token, ()> {
    let mut sign_bit = false;
    if s.starts_with("-") {
        sign_bit = true;
        s = &s[1..];
    }
    match map_res(
        alt((
            tuple((
                alt((tag("0x"), tag("0X"))),
                recognize(many1(terminated(
                    one_of("0123456789abcdefABCDEF"),
                    many0(char('_')),
                ))),
            )),
            tuple((
                tag("#"),
                recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))),
            )),
        )),
        |(tag, out)| {
            u16::from_str_radix(
                &str::replace(out, "_", ""),
                if tag.to_lowercase() == "0x" { 16 } else { 10 },
            )
        },
    )(s)
    {
        Ok((rem, value)) => Ok((rem, Token::IntegerOperand { value, sign_bit })),
        Err(e) => Err(e),
    }
}

/// Tests for parser
#[cfg(test)]
mod tests {
    use super::*;
    use crate::opcode::OpCode;

    #[test]
    fn test_parse_register() {
        let result = register("$0 ");
        assert_eq!(result.is_ok(), true);
        let result = register("0 ");
        assert_eq!(result.is_ok(), false);
        let result = register("$a ");
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_parse_integer_operand() {
        // Test a valid integer operand
        let result = integer_operand("#10 ");
        assert_eq!(result.is_ok(), true);
        let (rest, tok) = result.unwrap();
        assert_eq!(rest, " ");
        assert_eq!(
            tok,
            Token::IntegerOperand {
                value: 10,
                sign_bit: false
            }
        );

        // Test an invalid one (missing the #)
        let result = integer_operand("10 ");
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_parse_opcode() {
        // First tests that the opcode is detected and parsed correctly
        let result = opcode("load ");
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: OpCode::LOAD });
        assert_eq!(rest, " ");

        // Tests that an invalid opcode isn't recognized
        let result = opcode("aold ");
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_program_to_bytes() {
        let result = program("load $0 #100\n");
        assert_eq!(result.is_ok(), true);
        let (_, program) = result.unwrap();
        let bytecode = program.to_bytes();
        assert_eq!(bytecode.len(), 4);
        println!("{:?}", bytecode);
    }

    #[test]
    fn test_str_to_opcode() {
        let opcode = OpCode::from("load");
        assert_eq!(opcode, OpCode::LOAD);
        let opcode = OpCode::from("illegal");
        assert_eq!(opcode, OpCode::IGL);
    }
}
