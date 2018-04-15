use automata_core::string_interning::*;

/// A statement is a rule that maps a pattern to a destination
#[derive(Debug)]
pub struct Statement {
    /// The match pattern
    match_kind: StatementMatchKind,
    /// The destination state's name
    destination: InternedString,


}

impl Statement {
    pub fn new(destination: InternedString, match_kind: StatementMatchKind) -> Self {
        Self { destination, match_kind }
    }
}

/// A kind of statement
/// represents a pattern to match
#[derive(Debug)]
pub enum StatementMatchKind {
    Literal(char),
    Range(char, char),
    Default,
}
