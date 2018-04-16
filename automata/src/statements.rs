use automata_core::string_interning::*;

/// A statement is a rule that maps a pattern to a destination
#[derive(Debug)]
pub struct Statement {
    /// The match pattern
    pub match_kind: StatementMatchKind,
    /// The destination state's name
    pub destination: Destination,
}

impl Statement {
    /// Create a new statement with destination and match kind
    pub fn new(destination: Destination, match_kind: StatementMatchKind) -> Self {
        Self {
            destination,
            match_kind,
        }
    }
}

/// A State Destination
#[derive(Debug, Copy, Clone)]
pub enum Destination {
    State(InternedString),
    Return(InternedString),
}

/// A kind of statement
/// represents a pattern to match
#[derive(Debug)]
pub enum StatementMatchKind {
    Literal(char),
    Range(CharRange),
    Sequence(Vec<char>),
    Default,
}

/// Range from one character to another
#[derive(Debug, Copy, Clone)]
pub struct CharRange {
    from: char,
    to: char,
}

impl CharRange {
    /// Create a new CharRange
    /// Goes from `from` to `to`
    pub fn new(from: char, to: char) -> Self {
        CharRange { from, to }
    }
}

impl IntoIterator for CharRange {
    type Item = char;
    type IntoIter = CharRangeIntoIterator;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        CharRangeIntoIterator {
            range: self,
            current_char: self.from,
        }
    }
}

/// Iterator for a CharRange
pub struct CharRangeIntoIterator {
    range: CharRange,
    current_char: char,
}

impl Iterator for CharRangeIntoIterator {
    type Item = char;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        use std::u8;
        let return_char = self.current_char;
        let current_char_u8 = return_char as u8;

        let next_char = (current_char_u8 + 1) as char;

        if return_char <= self.range.to {
            self.current_char = next_char;
            return Some(return_char);
        }

        None
    }
}
