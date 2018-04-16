use std::collections::HashMap;
use statements::Destination;
use automata_core::string_interning::*;

#[derive(Debug)]
pub struct TransitionTable {
	transitions: HashMap<char, Vec<InternedString>>,
	return_states: HashMap<char, InternedString>,
	default_transition: Option<Destination>
}

impl TransitionTable {
	pub fn new() -> Self {
		TransitionTable {
			transitions: HashMap::new(),
			default_transition: None,
			return_states: HashMap::new()
		}
	}

	pub fn add_transition(&mut self, input: char, destination: InternedString) {
		let entry = self.transitions.entry(input).or_default();
		entry.push(destination);
	}

	pub fn add_return_state(&mut self, input: char, state: InternedString) {
		self.return_states.insert(input, state);
	}

	pub fn add_destination(&mut self, input: char, destination: Destination) {
		match destination {
			Destination::State(state) => self.add_transition(input, state),
			Destination::Return(return_state) => self.add_return_state(input, return_state)
		}
	}

	pub fn set_default_transition(&mut self, destination: Destination) {
		self.default_transition = Some(destination)
	}
}