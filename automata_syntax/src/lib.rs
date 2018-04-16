#![feature(nll)]
extern crate automata;
extern crate automata_core;
extern crate automata_parser;
extern crate colored;

#[macro_use]
extern crate lazy_static;

mod errors;

use automata_core::string_interning::*;
use automata::{statements::*, states::*};
use errors::*;
use automata_parser::tokens::*;
use automata_core::string_interning::InternedString;

lazy_static! {
    static ref KEYWORD_RETURN: InternedString = {
        return intern("return");
    };
}

/// Parses the syntax of some input
/// Uses automata_parser as the token parser / generator
pub struct SyntaxParser<'input> {
    parser: automata_parser::AutomataParser<'input>,
    input: &'input str,
}

impl<'input> SyntaxParser<'input> {
    /// Create a new SyntaxParser for some input string
    pub fn new(input: &'input str) -> Self {
        Self {
            parser: automata_parser::AutomataParser::new(input),
            input,
        }
    }

    /// Parses the SyntaxParser's input
    /// Returns a Vec of StateDefinition's
    pub fn parse(&mut self) -> Vec<StateDefinition> {
        let mut state_definitions : Vec<StateDefinition> = Vec::new();

        while let Some(token) = self.parser.get_next_token() {
            match token.kind {
                TokenKind::Identifier(name) => {
                    let mut state_definition = self.parse_state_definition(token, name);
                    state_definitions.append(&mut state_definition);
                }
                _ => {
                    syntax_err(self, "Did not expect at start of definition", &token);
                }
            }
        }

        return state_definitions;
    }

    /// Parse a StateDefinition given a token and a name
    fn parse_state_definition(&mut self, token: Token, name: InternedString) -> Vec<StateDefinition> {
        let mut current_state_definition = StateDefinition::new(name);
        let mut result = Vec::new();

        let open_token = self.parser.get_next_token();
        if let Some(open_token) = open_token {
            if let Token {
                kind: TokenKind::Scope(ScopeType::Open),
                ..
            } = open_token
            {
                while let Some(token) = self.parser.get_next_token() {
                    match token.kind.clone() {
                        TokenKind::Scope(ScopeType::Close) => {
                            result.push(current_state_definition);
                            return result;
                        }
                        TokenKind::UnderScore => {
                            let arrow_token = self.parser.get_next_token();

                            // The rest of the statement is `=> destination`
                            if let Some(arrow_token) = arrow_token {
                                if let Token {
                                    kind: TokenKind::Arrow,
                                    ..
                                } = arrow_token
                                {
                                    if let Some(destination) = self.parse_destination(&arrow_token) {
                                        current_state_definition.push_statement(Statement::new(
                                            destination,
                                            StatementMatchKind::Default,
                                        ));
                                    } else {
                                        syntax_err(
                                            self,
                                            "Could find valid destination after ",
                                            &arrow_token,
                                        );
                                    }
                                } else {
                                    syntax_err(self, "Expected arrow instead of ", &arrow_token);
                                }
                            } else {
                                syntax_err(self, "Expected arrow after", &token);
                            }
                        }
                        TokenKind::CharSequence(sequence) => {
                            let arrow_token = self.parser.get_next_token();

                            // The rest of the statement is `=> destination`
                            if let Some(arrow_token) = arrow_token {
                                if let Token {
                                    kind: TokenKind::Arrow,
                                    ..
                                } = arrow_token
                                    {
                                        if let Some(destination) = self.parse_destination(&arrow_token) {

                                            let mut intermediate_states: Vec<StateDefinition> = Vec::new();

                                            let sequence_as_str = sequence.iter().collect::<String>();

                                            'sequence: for i in 0..sequence.len() {
                                                let input = sequence[i];

												let destination_name = match destination {
                                                    Destination::State(interned_string) => {
                                                        format!("{}", intern_get_str(interned_string).unwrap())
                                                    }
                                                    Destination::Return(interned_string) => {
                                                        format!("return_{}", intern_get_str(interned_string).unwrap())
                                                    }
                                                };

                                                let intermediate_state_name = intern(format!("{}_to_{}_intermediate_{}_for_{}", intern_get_str(name).unwrap(), destination_name, i, sequence_as_str));

                                                match i {
                                                    0 => {
                                                        current_state_definition.push_statement(Statement::new(
                                                            Destination::State(intermediate_state_name),
                                                            StatementMatchKind::Literal(input),
                                                        ));
                                                    }
                                                    _ if i == sequence.len()-1 => {
                                                        intermediate_states[i-1].push_statement(Statement::new(
                                                            destination,
                                                            StatementMatchKind::Literal(input)
                                                        ));
                                                        break 'sequence;
                                                    }
                                                    _ => {
                                                        intermediate_states[i-1].push_statement(Statement::new(
                                                            Destination::State(intermediate_state_name),
                                                            StatementMatchKind::Literal(input)
                                                        ))
                                                    }
                                                }


                                                let intermediate_state = StateDefinition::new(intermediate_state_name);
                                                intermediate_states.push(intermediate_state);
                                            }

                                            result.append(&mut intermediate_states);


                                        } else {
                                            syntax_err(
                                                self,
                                                "Could find valid destination after ",
                                                &arrow_token,
                                            );
                                        }
                                    } else {
                                    syntax_err(self, "Expected arrow instead of ", &arrow_token);
                                }
                            } else {
                                syntax_err(self, "Expected arrow after", &token);
                            }
                        }
                        // it's currently assumed every statement starts with a char literal since it's the only thing supported
                        TokenKind::Char(chr) => {
                            let next_token = self.parser.get_next_token();

                            let mut arrow_token = None;

                            // It's assumed this is a literal but if a range is present, it can be changed later
                            let mut statement_kind = StatementMatchKind::Literal(chr);

                            if let Some(next_token) = next_token {
                                match next_token.kind {
                                    TokenKind::Range => {
                                        if let Some(token) = self.parser.get_next_token() {
                                            if let Token {
                                                kind: TokenKind::Char(to_chr),
                                                ..
                                            } = token
                                            {
                                                // A range token was present between two char literals, so we change the kind to a range
                                                statement_kind = StatementMatchKind::Range(
                                                    CharRange::new(chr, to_chr),
                                                );
                                            } else {
                                                syntax_err(self, "Invalid range close", &token);
                                            }
                                        } else {
                                            syntax_err(
                                                self,
                                                "Expected range close after",
                                                &next_token,
                                            );
                                        }
                                    }
                                    _ => {
                                        // Since the token wasn't a range, we assume it's an arrow and set it as so
                                        // If it is not an arrow, that error will be caught below
                                        arrow_token = Some(next_token.clone());
                                    }
                                }
                                if arrow_token == None {
                                    arrow_token = self.parser.get_next_token();
                                }

                                // The rest of the statement is `=> destination`
                                if let Some(arrow_token) = arrow_token {
                                    if let Token {
                                        kind: TokenKind::Arrow,
                                        ..
                                    } = arrow_token
                                    {
                                        if let Some(destination) =
                                            self.parse_destination(&arrow_token)
                                        {
                                            current_state_definition.push_statement(
                                                Statement::new(destination, statement_kind),
                                            );
                                        } else {
                                            syntax_err(
                                                self,
                                                "Could find valid destination after ",
                                                &arrow_token,
                                            );
                                        }
                                    } else {
                                        syntax_err(
                                            self,
                                            "Expected arrow instead of ",
                                            &arrow_token,
                                        );
                                    }
                                } else {
                                    syntax_err(self, "Expected arrow after", &next_token);
                                }
                            } else {
                                syntax_err(self, "Expected some token after", &token);
                            }
                        }
                        _ => {
                            syntax_err(self, "Statement cannot start with", &token);
                        }
                    }
                }
            } else {
                syntax_err(
                    self,
                    format!("Expected an open token after {:?}", token),
                    &open_token,
                );
            }
        } else {
            syntax_err(self, "Expected an open token after", &token);
        }

        result.push(current_state_definition);
        return result;
    }

    fn parse_destination(&mut self, token: &Token) -> Option<Destination> {
        if let Some(destination_token) = self.parser.get_next_token() {
            if let Token {
                kind: TokenKind::Identifier(destination),
                ..
            } = destination_token
            {
                if destination == *KEYWORD_RETURN {
                    if let Some(return_identifier_token) = self.parser.get_next_token() {
                        if let Token {
                            kind: TokenKind::Identifier(return_identifier),
                            ..
                        } = return_identifier_token
                        {
                            return Some(Destination::Return(return_identifier));
                        } else {
                            syntax_err(
                                self,
                                "Expected identifier after return",
                                &return_identifier_token,
                            );
                        }
                    } else {
                        syntax_err(self, "Expected identifier after return", &destination_token);
                    }
                } else {
                    return Some(Destination::State(destination));
                }
            } else {
                syntax_err(self, "Expected destination identifier", &destination_token);
            }
        } else {
            syntax_err(self, "Expected destination identifier after", &token);
        }

        None
    }
}
