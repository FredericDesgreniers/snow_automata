#![feature(entry_or_default)]

extern crate automata_core;

#[macro_use]
extern crate lazy_static;

pub mod statements;
pub mod states;
pub mod machine;

use machine::*;
use states::*;
use statements::*;
use automata_core::string_interning::*;
use std::collections::HashMap;

lazy_static! {
    static ref KEYWORD_SELF: InternedString = {
        return intern("Self");
    };
}

/// A state machine
#[derive(Debug)]
pub struct Automata {
    state_table: HashMap<InternedString, TransitionTable>,
}

impl Automata {
    /// Create an automata from a series of state definitions
    pub fn resolve_from(state_definitions: Vec<StateDefinition>) -> Self {
        let mut state_table = HashMap::new();

        for definition in state_definitions {
            let mut transition_table = TransitionTable::new();

            for statement in definition.statements {
                let mut destination = statement.destination;

                if let Destination::State(destination_identifier) = destination {
                    if destination_identifier == *KEYWORD_SELF {
                        destination = Destination::State(definition.name);
                    }
                }

                match statement.match_kind {
                    StatementMatchKind::Default => {
                        transition_table.set_default_transition(destination);
                    }
                    StatementMatchKind::Range(range) => for chr in range {
                        transition_table.add_destination(chr, destination);
                    },
                    StatementMatchKind::Literal(chr) => {
                        transition_table.add_destination(chr, destination);
                    }
                }
            }

            state_table.insert(definition.name, transition_table);
        }

        return Automata { state_table };
    }
}
