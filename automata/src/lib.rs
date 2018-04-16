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
        let mut automata = Automata {
            state_table: HashMap::new(),
        };

        for definition in &state_definitions {
            automata.process_state(definition);
        }

        return automata;
    }

    fn process_state(&mut self, definition: &StateDefinition) {
        let mut transition_table = TransitionTable::new();

        for statement in &definition.statements {
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
                StatementMatchKind::Sequence(ref sequence) => {
                    self.process_sequence(
                        sequence,
                        &destination,
                        &mut transition_table,
                        definition.name,
                    );
                }
            }
        }

        self.state_table.insert(definition.name, transition_table);
    }

    fn process_sequence(
        &mut self,
        sequence: &Vec<char>,
        destination: &Destination,
        source_state_transitions: &mut TransitionTable,
        source_name: InternedString,
    ) {
        let mut intermediate_states: Vec<StateDefinition> = Vec::new();

        let sequence_as_str = sequence.iter().collect::<String>();

        'sequence: for i in 0..sequence.len() {
            let input = sequence[i];

            let destination_name = match destination {
                Destination::State(interned_string) => {
                    format!("{}", intern_get_str(*interned_string).unwrap())
                }
                Destination::Return(interned_string) => {
                    format!("return_{}", intern_get_str(*interned_string).unwrap())
                }
            };

            let intermediate_state_name = intern(format!(
                "{}_to_{}_intermediate_{}_for_{}",
                intern_get_str(source_name).unwrap(),
                destination_name,
                i,
                sequence_as_str
            ));

            match i {
                0 => {
                    source_state_transitions
                        .add_destination(input, Destination::State(intermediate_state_name));
                }
                _ if i == sequence.len() - 1 => {
                    intermediate_states[i - 1].push_statement(Statement::new(
                        *destination,
                        StatementMatchKind::Literal(input),
                    ));
                    break 'sequence;
                }
                _ => intermediate_states[i - 1].push_statement(Statement::new(
                    Destination::State(intermediate_state_name),
                    StatementMatchKind::Literal(input),
                )),
            }

            let intermediate_state = StateDefinition::new(intermediate_state_name);
            intermediate_states.push(intermediate_state);
        }

        intermediate_states.iter().for_each(|state_definition| {
            self.process_state(state_definition);
        })
    }
}
