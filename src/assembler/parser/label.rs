use nom::{bytes::complete::tag, character::complete::{alpha1, multispace1}, sequence::tuple, IResult, combinator::opt, character::complete::char};

use super::Token;


pub(in crate::assembler) fn label_declaration(s: &str) -> IResult<&str, Token, ()> {
    match tuple((alpha1, char(':')))(s) {
        Ok((rem, (name, _))) => Ok((
            rem,
            Token::LabelDeclaration {
                name: name.to_string(),
            },
        )),
        Err(e) => Err(e),
    }
}

pub(in crate::assembler) fn label_usage(s: &str) -> IResult<&str, Token, ()> {
    match tuple((char('@'), alpha1, opt(multispace1)))(s) {
        Ok((rem, (_, name, _))) => Ok((
            rem,
            Token::LabelUsage {
                name: name.to_lowercase(),
            },
        )),
        Err(e) => Err(e),
    }
}

/// Tests for label
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_label_declaration() {
        let result = label_declaration("test:");
        assert_eq!(result.is_ok(), true);
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::LabelDeclaration { name: "test".to_string() });
        let result = label_declaration("test");
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_parse_label_usage() {
        let result = label_usage("@test");
        assert_eq!(result.is_ok(), true);
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::LabelUsage { name: "test".to_string() });
        let result = label_usage("test");
        assert_eq!(result.is_ok(), false);
    }

}