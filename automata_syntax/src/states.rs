use super::statements::*;
use automata_core::string_interning::*;

/// A state definition
/// Contains the name and the contained statements
#[derive(Debug)]
pub struct StateDefinition {
	name: InternedString,
	statements: Vec<Statement>
}

impl StateDefinition {
	/// Create a new StateDefinition given it's name
	pub fn new(name: InternedString) -> Self {
		Self {
			name,
			statements: Vec::new()
		}
	}

	/// Push a new statement onto the StateDefinition
	pub fn push_statement(&mut self, statement: Statement) {
		self.statements.push(statement);
	}
}

