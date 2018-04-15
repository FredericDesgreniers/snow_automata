use std::collections::HashMap;
use statements::Destination;

#[derive(Debug)]
pub struct TransitionTable {
	transitions: HashMap<char, Vec<Destination>>,
	default_transition: Option<Destination>
}

impl TransitionTable {
	pub fn new() -> Self {
		TransitionTable {
			transitions: HashMap::new(),
			default_transition: None
		}
	}

	pub fn add_transition(&mut self, input: char, destination: Destination) {
		let entry = self.transitions.entry(input).or_default();
		entry.push(destination);
	}

	pub fn set_default_transition(&mut self, destination: Destination) {
		self.default_transition = Some(destination)
	}
}