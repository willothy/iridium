use crate::instruction::OpCode;
use nom::{
    character::complete::{alpha1, space1, digit1, newline, char},
    sequence::{terminated, tuple},
    IResult
};

#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: OpCode },
    Register { id: u8 },
    IntegerOperand { value: u16 },
    Newline,
}

fn opcode(s: &str) -> IResult<Token, &str, ()> {
    match terminated(alpha1, space1)(s) {
        Ok((rem, opcode)) => Ok((Token::Op { code: match OpCode::from(opcode.to_owned()) {
            OpCode::IGL => return Err(nom::Err::Error(())),
            opcode => opcode,
        } }, rem)),
        Err(e) => Err(e),
    }
}

fn number(s: &str) -> IResult<Token, &str, ()> {
    match digit1(s) {
        Ok((rem, number)) => Ok((Token::Register { id: number.parse().unwrap() }, rem)),
        Err(e) => Err(e),
    }
}

fn parse_newline(s: &str) -> IResult<Token, &str, ()> {
    match newline(s) {
        Ok((rem, _)) => Ok((Token::Newline, rem)),
        Err(e) => Err(e),
    }
}

fn register(s: &str) -> IResult<Token, (char, &str), ()> {
    match terminated(
        tuple(
            (char('$'),
            digit1),
        ),
        space1,
    )(s) {
        Ok((rem, (val, number))) => Ok((Token::Register { id: number.parse().unwrap() }, (val, rem))),
        Err(e) => Err(e),
    }
}

fn integer_operand(s: &str) -> IResult<Token, &str, ()> {
    match terminated(
        tuple(
            (
                char('#'),
                digit1
            ),
        ),
        space1,
    )(s) {
        Ok((rem, (val, number))) => Ok((Token::IntegerOperand { value: number.parse().unwrap() }, rem)),
        Err(e) => Err(e),
    }
}

/// Tests for lexer
#[cfg(test)]
mod tests {
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
        let (tok, rest) = result.unwrap();
        assert_eq!(rest, "");
        assert_eq!(tok, Token::IntegerOperand{value: 10});

        // Test an invalid one (missing the #)
        let result = integer_operand("10 ");
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_parse_opcode() {
        // First tests that the opcode is detected and parsed correctly
        let result = opcode("load ");
        assert_eq!(result.is_ok(), true);
        let (token, rest) = result.unwrap();
        assert_eq!(token, Token::Op{code: OpCode::LOAD});
        assert_eq!(rest, "");

        // Tests that an invalid opcode isn't recognized
        let result = opcode("aold ");
        assert_eq!(result.is_ok(), false);
    }

}
