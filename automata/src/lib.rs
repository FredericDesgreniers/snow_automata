#![feature(entry_or_default)]

extern crate automata_core;

pub mod statements;
pub mod states;
pub mod machine;

use machine::*;
use states::*;
use statements::*;
use automata_core::string_interning::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Automata {
    state_table: HashMap<InternedString, TransitionTable>
}

impl Automata {
    pub fn resolve_from(state_definitions: Vec<StateDefinition>) -> Self {
        let mut state_table = HashMap::new();

        for definition in state_definitions {
            let mut transition_table = TransitionTable::new();

            for statement in definition.statements {
                let destination = statement.destination;

                match statement.match_kind {
                    StatementMatchKind::Default => {
                        transition_table.set_default_transition(destination);
                    },
                    StatementMatchKind::Range(range) => {
                        for chr in range {
                            transition_table.add_transition(chr, destination);
                        }
                    },
                    StatementMatchKind::Literal(chr) => {
                        transition_table.add_transition(chr, destination);
                    }
                }
            }

            state_table.insert(definition.name, transition_table);
        }

        return Automata {
            state_table
        }
    }
}
