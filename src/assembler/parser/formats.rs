use nom::{sequence::{terminated, tuple}, IResult, character::complete::{space1, newline}};

use crate::assembler::instruction::AssemblerInstruction;

use super::parsers::{opcode, register, integer_operand};

/// OP $reg #value
pub(in super) fn instruction_one(s: &str) -> IResult<&str, AssemblerInstruction, ()> {
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
pub(in super) fn instruction_two(s: &str) -> IResult<&str, AssemblerInstruction, ()> {
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
pub(in super) fn instruction_three(s: &str) -> IResult<&str, AssemblerInstruction, ()> {
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

/// Tests for formats
#[cfg(test)]
mod tests {
    use crate::{assembler::parser::Token, instruction::OpCode};
    use super::*;

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