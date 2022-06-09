use nom::{
    character::complete::{newline, space1},
    sequence::{terminated, tuple},
    IResult, combinator::opt,
};

use crate::assembler::instruction::AssemblerInstruction;

use super::{parsers::*, label::label_declaration};

/// OP $reg #value
pub(in crate::assembler) fn op_reg_val(s: &str) -> IResult<&str, AssemblerInstruction, ()> {
    match terminated(
        tuple((opcode, space1, register, space1, integer_operand)),
        newline,
    )(s)
    {
        Ok((rem, (opcode, _, register, _, integer_operand))) => Ok((
            rem,
            AssemblerInstruction::new(
                Some(opcode),
                [Some(register), Some(integer_operand), None],
                None,
                None,
            ),
        )),
        Err(e) => Err(e),
    }
}

/// OP
pub(in crate::assembler) fn op(s: &str) -> IResult<&str, AssemblerInstruction, ()> {
    match terminated(opcode, newline)(s) {
        Ok((rem, opcode)) => Ok((
            rem,
            AssemblerInstruction::new(Some(opcode), [None, None, None], None, None),
        )),
        Err(e) => Err(e),
    }
}

/// OP $reg $reg $reg
pub(in crate::assembler) fn op_reg_reg_reg(s: &str) -> IResult<&str, AssemblerInstruction, ()> {
    match terminated(
        tuple((opcode, space1, register, space1, register, space1, register)),
        newline,
    )(s)
    {
        Ok((rem, (opcode, _, r0, _, r1, _, r2))) => Ok((
            rem,
            AssemblerInstruction::new(Some(opcode), [Some(r0), Some(r1), Some(r2)], None, None),
        )),
        Err(e) => Err(e),
    }
}

/// Op $reg
pub(in crate::assembler) fn op_reg(s: &str) -> IResult<&str, AssemblerInstruction, ()> {
    match terminated(tuple((opcode, space1, register)), newline)(s) {
        Ok((rem, (opcode, _, register))) => Ok((
            rem,
            AssemblerInstruction::new(Some(opcode), [Some(register), None, None], None, None),
        )),
        Err(e) => Err(e),
    }
}

/// OP $reg $reg
pub(in crate::assembler) fn op_reg_reg(s: &str) -> IResult<&str, AssemblerInstruction, ()> {
    match terminated(tuple((opcode, space1, register, space1, register)), newline)(s) {
        Ok((rem, (opcode, _, r0, _, r1))) => Ok((
            rem,
            AssemblerInstruction::new(Some(opcode), [Some(r0), Some(r1), None], None, None),
        )),
        Err(e) => Err(e),
    }
}

pub(in crate::assembler) fn instruction_combined(s: &str) -> IResult<&str, AssemblerInstruction, ()> {
    match tuple((opt(label_declaration), opcode, opt(operand), opt(operand), opt(operand)))(s)
    {
        Ok((rem, (label_dec, opcode, operand1, operand2, operand3))) => Ok((
            rem,
            AssemblerInstruction::new(
                Some(opcode),
                [operand1, operand2, operand3],
                label_dec,
                None
            )
        )),
        Err(e) => Err(e),
    }
}

/// Tests for formats
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assembler::parser::Token, opcode::OpCode};

    #[test]
    fn test_parse_instruction_form_one() {
        let result = op_reg_val("load $0 #100\n");
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
        let result = op("hlt\n");
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
