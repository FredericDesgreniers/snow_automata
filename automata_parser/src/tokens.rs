use automata_core::string_interning::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub debug_info: TokenDebugInfo,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TokenKind {
    Identifier(InternedString),
    Arrow,
    Column,
    Char(char),
    CharSequence(Vec<char>),
    Range,
    SemiColumn,
    Integer(i32),
    Scope(ScopeType),
    UnderScore,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ScopeType {
    Open,
    Close,
}

impl Token {
    pub fn new(kind: TokenKind, debug_info: TokenDebugInfo) -> Self {
        Self { kind, debug_info }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TokenDebugInfo {
    pub column_location: (usize, usize),
    pub line_location: (usize, usize),
    pub index_location: (usize, usize),
}

impl TokenDebugInfo {
    pub fn new(
        column_location: (usize, usize),
        line_location: (usize, usize),
        index_location: (usize, usize),
    ) -> Self {
        Self {
            column_location,
            line_location,
            index_location,
        }
    }
}
