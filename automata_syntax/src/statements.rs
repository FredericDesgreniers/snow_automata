use automata_core::string_interning::*;

/// A statement is a rule that maps a pattern to a destination
#[derive(Debug)]
pub struct Statement {
	/// The destination state's name
	destination: InternedString,
	/// The pattern
	kind: StatementKind
}

impl Statement {
	pub fn new(destination: InternedString, kind: StatementKind) -> Self {
		Self {
			destination,
			kind
		}
	}
}

/// A kind of statement
/// represents a pattern to match
#[derive(Debug)]
pub enum StatementKind {
	Literal(char),
	Range(char, char),
	Default
}
