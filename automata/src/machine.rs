use std::collections::HashMap;
use statements::Destination;
use automata_core::string_interning::*;

/// A transition table for a state machine
#[derive(Debug)]
pub struct TransitionTable {
    /// Normal char -> state transitions
    transitions: HashMap<char, Vec<InternedString>>,
    /// Accepting transitions
    return_states: HashMap<char, InternedString>,
    /// Default transition
    default_transition: Option<Destination>,
}

impl TransitionTable {
    /// Create a new TransitionTable
    pub fn new() -> Self {
        TransitionTable {
            transitions: HashMap::new(),
            default_transition: None,
            return_states: HashMap::new(),
        }
    }

    /// Add a normal transition
    pub fn add_transition(&mut self, input: char, destination: InternedString) {
        let entry = self.transitions.entry(input).or_default();
        entry.push(destination);
    }

    /// Add a return state
    pub fn add_return_state(&mut self, input: char, state: InternedString) {
        self.return_states.insert(input, state);
    }

    /// Add a destination
    /// Will dispatch to either normal transition or return state
    pub fn add_destination(&mut self, input: char, destination: Destination) {
        match destination {
            Destination::State(state) => self.add_transition(input, state),
            Destination::Return(return_state) => self.add_return_state(input, return_state),
        }
    }

    /// Set the default transition for a this table
    pub fn set_default_transition(&mut self, destination: Destination) {
        self.default_transition = Some(destination)
    }
}
