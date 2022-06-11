use super::Token;
use crate::assembler::{Program, instruction::AssemblerInstruction};
use crate::opcode::OpCode;

use nom::bytes::complete::take_until;
use nom::character::complete::{space1, newline, space0};
use nom::sequence::preceded;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::one_of,
    character::complete::{alpha1, char, digit1},
    combinator::{map_res, recognize, opt},
    multi::{many0, many1},
    sequence::{terminated, tuple},
    IResult,
};

pub fn program(s: &str) -> IResult<&str, Program, ()> {
    match many1(alt((instruction, directive)))(s)
    {
        Ok((rem, instructions)) => Ok((rem, Program { instructions })),
        Err(e) => Err(e),
    }
}

pub fn instruction(s: &str) -> IResult<&str, AssemblerInstruction, ()> {
    match terminated(
        tuple((
            opt(label_declaration),
            opcode,
            opt(preceded(space1, operand)),
            opt(preceded(space1, operand)),
            opt(preceded(space1, operand)),
        )),
        newline,
    )(s)
    {
        Ok((rem, (label_dec, opcode, operand1, operand2, operand3))) => Ok((
            rem,
            AssemblerInstruction::new(
                Some(opcode),
                [operand1, operand2, operand3],
                label_dec,
                None,
            ),
        )),
        Err(e) => Err(e),
    }
}

pub fn operand(s: &str) -> IResult<&str, Token, ()> {
    match alt((register, integer_operand, label_usage, irstring))(s) {
        Ok((rem, token)) => Ok((rem, token)),
        Err(e) => Err(e),
    }
}

pub fn opcode(s: &str) -> IResult<&str, Token, ()> {
    match alpha1(s) {
        Ok((rem, opcode)) => Ok((
            rem,
            Token::Op {
                code: match OpCode::from_string(&opcode.to_lowercase()[..]) {
                    /* Ok(opcode) => {
                        match opcode {
                            
                        }
                    }
                    Err(e) => return Err(nom::Err::Error(())) */
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

pub fn label_declaration(s: &str) -> IResult<&str, Token, ()> {
    match tuple((alpha1, char(':'), space0, opt(newline)))(s) {
        Ok((rem, (name, _, _, _))) => Ok((
            rem,
            Token::LabelDeclaration {
                name: name.to_string(),
            },
        )),
        Err(e) => Err(e),
    }
}

pub fn label_usage(s: &str) -> IResult<&str, Token, ()> {
    // Parsing a label usage.
    match tuple((char('@'), alpha1))(s) {
        Ok((rem, (_, name))) => Ok((
            rem,
            Token::LabelUsage {
                name: name.to_lowercase(),
            },
        )),
        Err(e) => Err(e),
    }
}

pub fn directive(s: &str) -> IResult<&str, AssemblerInstruction, ()> {
    match terminated(tuple((
        char('.'),
        alpha1,
        opt(operand),
        opt(operand),
        opt(operand),
        space0
    )), newline)(s)
    {
        Ok((rem, (_, directive, operand1, operand2, operand3, _))) => Ok((
            rem,
            AssemblerInstruction {
                opcode: None,
                operands: [operand1, operand2, operand3],
                directive: Some(Token::Directive { name: directive.to_lowercase() }),
                label: None,
            },
        )),
        Err(e) => Err(e),
    }
}

pub fn irstring(s: &str) -> IResult<&str, Token, ()> {
    match tuple(
        (char('\''), take_until("'"), char('\'')))(s)
    {
        Ok((rem, (_, content, _))) => Ok((
            rem,
            Token::IRString {
                name: content.to_owned(),
            },
        )),
        Err(e) => Err(e),
    }
}

/// Tests for parser
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{opcode::OpCode, assembler::SymbolTable};

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
        let (rest, program) = result.unwrap();
        println!("{}", rest);
        let bytecode = program.to_bytes(&SymbolTable::new());
        println!("{:?}", bytecode);
        assert_eq!(bytecode.len(), 4);
        println!("{:?}", bytecode);
    }

    #[test]
    fn test_str_to_opcode() -> Result<(), strum::ParseError> {
        let opcode = OpCode::from_string("load");
        assert_eq!(opcode, OpCode::LOAD);
        let opcode = OpCode::from_string("illegal");
        assert_eq!(opcode, OpCode::IGL);
        Ok(())
    }

    #[test]
    fn test_parse_label_declaration() {
        let result = label_declaration("test:");
        assert_eq!(result.is_ok(), true);
        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::LabelDeclaration {
                name: "test".to_string()
            }
        );
        let result = label_declaration("test");
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_parse_label_usage() {
        let result = label_usage("@test");
        assert_eq!(result.is_ok(), true);
        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::LabelUsage {
                name: "test".to_string()
            }
        );
        let result = label_usage("test");
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_parse_instruction_form_one() {
        let result = instruction("load $0 #100\n");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblerInstruction::new(
                    Some(Token::Op { code: OpCode::LOAD }),
                    [
                        Some(Token::Register { id: 0 }),
                        Some(Token::IntegerOperand {
                            value: 100,
                            sign_bit: false
                        }),
                        None
                    ],
                    None,
                    None
                )
            ))
        );
    }

    #[test]
    fn test_parse_instruction_form_two() {
        let result = instruction("hlt\n");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblerInstruction::new(
                    Some(Token::Op { code: OpCode::HLT }),
                    [None, None, None],
                    None,
                    None
                )
            ))
        );
    }
}
