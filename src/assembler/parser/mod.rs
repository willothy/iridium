pub mod parsers;

mod token;
pub use token::*;

use super::Program;

pub fn parse_program(s: &str) -> Result<Program, nom::Err<()>> {
    Ok(super::parser::parsers::program(s)?.1)
}
