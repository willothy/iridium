pub mod instruction;
pub mod parser;

pub mod program;
pub use program::Program;

use self::instruction::AssemblerInstruction;

#[derive(Debug, PartialEq)]
pub enum AssemblerPhase {
    First,
    Second,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AssemblerError {
    NoSegmentDeclarationFound { instruction: u32 },
    StringConstantDeclaredWithoutLabel { instruction: u32 },
    SymbolAlreadyDeclared,
    UnknownDirectiveFound { directive: String },
    NonOpcodeInOpcodeField,
    InsufficientSections,
    ParseError { error: String }
}

impl std::fmt::Display for AssemblerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AssemblerError::NoSegmentDeclarationFound { instruction } => {
                write!(f, "No segment declaration found for instruction {}", instruction)
            }
            AssemblerError::StringConstantDeclaredWithoutLabel { instruction } => {
                write!(f, "String constant declared without label in instruction {}", instruction)
            }
            AssemblerError::SymbolAlreadyDeclared => {
                write!(f, "Symbol already declared")
            }
            AssemblerError::UnknownDirectiveFound { directive } => {
                write!(f, "Unknown directive found: {}", directive)
            }
            AssemblerError::NonOpcodeInOpcodeField => {
                write!(f, "Non-opcode in opcode field")
            }
            AssemblerError::InsufficientSections => {
                write!(f, "Insufficient sections")
            }
            AssemblerError::ParseError { error } => {
                write!(f, "Parse error: {}", error)
            }
        }
    }
}

#[derive(Debug)]
pub struct Assembler {
    pub phase: AssemblerPhase,
    pub symbols: SymbolTable,
    pub ro: Vec<u8>,
    pub bytecode: Vec<u8>,
    ro_offset: u32,
    sections: Vec<AssemblerSection>,
    current_section: Option<AssemblerSection>,
    current_instruction: u32,
    errors: Vec<AssemblerError>
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            phase: AssemblerPhase::First,
            symbols: SymbolTable::new(),
            ro: vec![],
            bytecode: vec![],
            ro_offset: 0,
            sections: vec![],
            current_section: None,
            current_instruction: 0,
            errors: vec![],
        }
    }

    pub fn assemble(&mut self, raw: &str) -> Result<&mut Vec<u8>, Vec<AssemblerError>> {
        match program::parse_program(raw) {
            Ok(program) => {
                self.process_first_phase(&program);
                if !self.errors.is_empty() {
                    return Err(self.errors.clone());
                }
                if self.sections.len() != 2  {
                    println!("Did not find at least two sections");
                    self.errors.push(AssemblerError::InsufficientSections);
                    return Err(self.errors.clone());
                }
                self.process_second_phase(&program);
                Ok(&mut self.bytecode)
            }
            Err(e) => {
                println!("{}", e);
                self.errors.push(AssemblerError::ParseError { error: e.to_string() });
                Err(self.errors.clone())
            }
        }
    }

    fn process_first_phase(&mut self, p: &Program) {
        //self.extract_labels(p);
        for i in &p.instructions {
            if i.is_label() {
                if self.current_section.is_some() {
                    self.process_label_declaration(&i);
                } else {
                    self.errors.push(AssemblerError::NoSegmentDeclarationFound{instruction: self.current_instruction});
                }
            }
            if i.is_directive() {
                self.process_directive(&i);
            }
            self.current_instruction += 1;
        }
        self.phase = AssemblerPhase::Second;
    }

    fn process_second_phase(&mut self, p: &Program) {
        let mut program: Vec<u8> = vec![];
        for i in &p.instructions {
            if i.is_instruction() {
                program.append(&mut i.to_bytes(&self.symbols))
            }
            self.current_instruction += 1;
        }
        self.bytecode.append(&mut program);
    }

    fn process_label_declaration(&mut self, i: &AssemblerInstruction) {
        let name = match i.label_name() {
            Some(name) => {
                name
            },
            None => {
                self.errors.push(AssemblerError::StringConstantDeclaredWithoutLabel{instruction: self.current_instruction});
                return;
            }
        };

        if self.symbols.has_symbol(&name) {
            self.errors.push(AssemblerError::SymbolAlreadyDeclared);
            return;
        }
        let section_start = match self.current_section {
            Some(AssemblerSection::Code { starting_instruction }) => {
                starting_instruction
            }
            Some(AssemblerSection::Data { starting_instruction }) => {
                starting_instruction
            }
            Some(AssemblerSection::Unknown) => {
                return;
            }
            None => {
                self.errors.push(AssemblerError::NoSegmentDeclarationFound{instruction: self.current_instruction});
                return;
            }
        };
        self.symbols.add_symbol(Symbol::new(name, SymbolType::Label, (self.current_instruction * 4) - (section_start + 4)));
    }

    fn process_directive(&mut self, i: &AssemblerInstruction) {
        let directive_name = match i.directive_name() {
            Some(name) => name,
            None => ""
        };
        if i.has_operands() {
            match directive_name {
                "asciiz" => {
                    self.handle_asciiz(i);
                }
                _ => {
                    self.errors.push(AssemblerError::UnknownDirectiveFound{directive: directive_name.to_owned()});
                    return;
                }
            }
        } else {
            self.process_section_header(&directive_name);
        }
    }

    fn process_section_header(&mut self, name: &str) {
        let new_section: AssemblerSection = AssemblerSection::new(name, self.current_instruction * 4);

        if new_section == AssemblerSection::Unknown {
            println!("Found an unknown section header: {:#?}", name);
            return;
        }

        self.sections.push(new_section.clone());
        self.current_section = Some(new_section);
    }

    fn handle_asciiz(&mut self, i: &AssemblerInstruction) {
        if self.phase != AssemblerPhase::First {
            return
        }
        if let Some(_) = i.get_string_constant() {
            match i.label_name() {
                Some(name) => self.symbols.set_symbol_offset(&name, self.ro_offset),
                None => {
                    println!("Found string const without associated label");
                    return
                }
            }
        } else {
            println!("String constant following an .asciiz was empty");
            return
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum AssemblerSection {
    Data { starting_instruction: u32 },
    Code { starting_instruction: u32 },
    Unknown
}

impl Default for AssemblerSection {
    fn default() -> AssemblerSection {
        AssemblerSection::Unknown
    }
}

impl AssemblerSection {
    pub fn new(name: &str, starting_instruction: u32) -> Self {
        match name {
            "data" => AssemblerSection::Data { starting_instruction },
            "code" => AssemblerSection::Code { starting_instruction },
            _ => AssemblerSection::Unknown
        }
    }
}

/* impl From<&str> for AssemblerSection {
    fn from(s: &str) -> AssemblerSection {
        match s {
            "data" => AssemblerSection::Data { starting_instruction: 0 },
            "code" => AssemblerSection::Code { starting_instruction: 0 },
            _ => AssemblerSection::Unknown
        }
    }
} */

#[derive(Debug)]
pub struct Symbol {
    name: String,
    offset: Option<u32>,
    symbol_type: SymbolType,
}

impl Symbol {
    pub fn new(name: &str, symbol_type: SymbolType, offset: u32) -> Self {
        Self {
            name: name.to_string(),
            offset: Some(offset),
            symbol_type,
        }
    }
}

#[derive(Debug)]
pub enum SymbolType {
    Label,
}

#[derive(Debug)]
pub struct SymbolTable {
    symbols: Vec<Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self { symbols: vec![] }
    }

    pub fn add_symbol(&mut self, symbol: Symbol) {
        self.symbols.push(symbol);
    }

    pub fn has_symbol(&self, name: &str) -> bool {
        self.symbols.iter().any(|s| s.name == name)
    }

    pub fn get_symbol_offset(&self, name: &str) -> Option<u32> {
        for symbol in &self.symbols {
            if symbol.name == name {
                return symbol.offset;
            }
        }
        None
    }

    pub fn set_symbol_offset(&mut self, name: &str, offset: u32) {
        for symbol in &mut self.symbols {
            if symbol.name == name {
                symbol.offset = Some(offset);
            }
        }
    }
}

/// Tests for mod
#[cfg(test)]
mod tests {
    use crate::vm::VM;

    use super::*;

    #[test]
    fn test_symbol_table() {
        let mut sym = SymbolTable::new();
        let new_symbol = Symbol::new("test", SymbolType::Label, 12);
        sym.add_symbol(new_symbol);
        assert_eq!(sym.symbols.len(), 1);
        let v = sym.get_symbol_offset("test");
        assert_eq!(true, v.is_some());
        let v = v.unwrap();
        assert_eq!(v, 12);
        let v = sym.get_symbol_offset("does_not_exist");
        assert_eq!(v.is_some(), false);
    }

    #[test]
    fn test_assemble_program() -> Result<(), Vec<AssemblerError>> {
        let mut asm = Assembler::new();
        let test_string = r".data
.code
load $0 #100
load $1 #1
load $2 #0
test: inc $0
neq $0 $2
jeq @test
hlt
";
        let program = match asm.assemble(test_string) {
            Ok(p) => p,
            Err(e) => {
                println!("{:?}", asm.symbols.symbols);
                return Err(e)
            }
        };
        let mut vm = VM::new();
        assert_eq!(program.len(), 28);
        vm.add_program(program);
        assert_eq!(vm.read_program().len(), 28);
        Ok(())
    }
}
