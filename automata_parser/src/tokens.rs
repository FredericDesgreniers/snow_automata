use automata_core::string_interning::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub column_location: (usize, usize),
    pub line_location: (usize, usize),
    pub index_location: (usize, usize)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TokenKind {
    Identifier(InternedString),
    Arrow,
    Column,
    Char(char),
    Range,
    SemiColumn,
    Integer(i32),
    Scope(ScopeType),
    UnderScore
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ScopeType {
    Open,
    Close
}

impl Token {
    pub fn new(kind: TokenKind, column_location: (usize, usize), line_location: (usize, usize), index_location: (usize, usize)) -> Self {
        Self {
            kind,
            line_location,
            column_location,
            index_location
        }
    }
}
