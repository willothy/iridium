use crate::instruction::OpCode;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::one_of,
    character::complete::{alpha1, char, digit1, newline, space1},
    combinator::{map_res, recognize},
    multi::{many0, many1},
    sequence::{preceded, terminated, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: OpCode },
    Register { id: u8 },
    IntegerOperand { value: u16, sign_bit: bool },
    //Number { value: u16 },
}

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    opcode: Token,
    operands: [Option<Token>; 3],
}

impl AssemblerInstruction {
    pub fn new(opcode: Token, operands: [Option<Token>; 3]) -> Self {
        Self { opcode, operands }
    }

    fn to_bytes(&self) -> Vec<u8> {
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

#[derive(Debug, PartialEq)]
pub struct Program {
    instructions: Vec<AssemblerInstruction>,
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
    Ok(self::program(s)?.1)
}

fn program(s: &str) -> IResult<&str, Program, ()> {
    match many1(alt((instruction_one, instruction_two, instruction_three)))(s) {
        Ok((rem, instructions)) => Ok((rem, Program { instructions })),
        Err(e) => Err(e),
    }
}

fn opcode(s: &str) -> IResult<&str, Token, ()> {
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

fn register(s: &str) -> IResult<&str, Token, ()> {
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

fn integer_operand(mut s: &str) -> IResult<&str, Token, ()> {
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
                )))
            )),
            tuple((
                tag("#"),
                recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))),
            )),
        )),
        |(tag, out)| u16::from_str_radix(&str::replace(out, "_", ""), if tag.to_lowercase() == "0x" { 16 } else { 10 }),
    )(s)
    {
        Ok((rem, value)) => Ok((rem, Token::IntegerOperand { value, sign_bit })),
        Err(e) => Err(e),
    }
}


/// OP $reg #value
fn instruction_one(s: &str) -> IResult<&str, AssemblerInstruction, ()> {
    match terminated(
        tuple((opcode, space1, register, space1, integer_operand)),
        newline,
    )(s)
    {
        Ok((rem, (opcode, _, register, _, integer_operand))) => Ok((
            rem,
            AssemblerInstruction::new(opcode, [Some(register), Some(integer_operand), None]),
        )),
        Err(e) => Err(e),
    }
}

/// OP
fn instruction_two(s: &str) -> IResult<&str, AssemblerInstruction, ()> {
    match terminated(
        opcode,
        newline,
    )(s)
    {
        Ok((rem, opcode)) => Ok((
            rem,
            AssemblerInstruction::new(opcode, [None, None, None]),
        )),
        Err(e) => Err(e),
    }
}

/// OP $reg $reg $reg
fn instruction_three(s: &str) -> IResult<&str, AssemblerInstruction, ()> {
    match terminated(
        tuple((opcode, space1, register, space1, register, space1, register)),
        newline,
    )(s)
    {
        Ok((rem, (opcode, _, r0, _, r1, _, r2))) => Ok((
            rem,
            AssemblerInstruction::new(opcode, [Some(r0), Some(r1), Some(r2)]),
        )),
        Err(e) => Err(e),
    }
}

/// Tests for parser
#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::OpCode;

    use super::*;

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
        assert_eq!(tok, Token::IntegerOperand { value: 10, sign_bit: false });

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
    fn test_parse_instruction_form_one() {
        let result = instruction_one("load $0 #100\n");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblerInstruction::new(
                    Token::Op { code: OpCode::LOAD },
                    [
                        Some(Token::Register { id: 0 }),
                        Some(Token::IntegerOperand { value: 100, sign_bit: false }),
                        None
                    ]
                )
            ))
        );
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

    #[test]
    fn test_parse_instruction_form_two() {
        let result = instruction_two("hlt\n");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblerInstruction {
                    opcode: Token::Op { code: OpCode::HLT },
                    operands: [None, None, None]
                }
            ))
        );
    }
}
