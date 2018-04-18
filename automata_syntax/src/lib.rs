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
    static ref KEYWORD_STATE: InternedString = {
        return intern("state");
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
        let mut state_definitions: Vec<StateDefinition> = Vec::new();

        while let Some(token) = self.parser.get_next_token() {
            match token.kind.clone() {
                TokenKind::Identifier(name) => {
                    if name == *KEYWORD_STATE {
                        let mut state_definition = self.parse_state_definition(token);
                        state_definitions.append(&mut state_definition);
                    } else {
                        syntax_err(self, "Could not start a definition with", &token)
                    }
                }
                _ => {
                    syntax_err(self, "Did not expect at start of definition", &token);
                }
            }
        }

        return state_definitions;
    }

    /// Parse a StateDefinition given a token and a name
    fn parse_state_definition(
        &mut self,
        token: Token,
    ) -> Vec<StateDefinition> {

        let name = if let Some(token) = self.parser.get_next_token() {
            match token.kind {
                TokenKind::Identifier(identifier) => {
                    identifier
                }
                _ => {
                    intern("")
                }
            }
        } else {
            syntax_err(self, "Expected a state name after", &token);
            intern("no name provided")
        };

        let mut current_state_definition = StateDefinition::new(name);
        let mut result = Vec::new();

        let open_token = self.parser.get_next_token();
        if let Some(open_token) = open_token {
            if let Token {
                kind: TokenKind::Scope(ScopeType::Open),
                ..
            } = open_token
            {
                'statements: loop {
                    let (match_statements, next_token) = self.parse_left_side_inputs();
                    if match_statements.is_empty() {
                        if let Some(next_token) = next_token {
                            match next_token.kind.clone() {
                                TokenKind::Scope(ScopeType::Close) => {
                                    // This is ok, but we don't do anything since we want to break even on error
                                }
                                _ => {
                                    syntax_err(self, "State did not close", &next_token);
                                }
                            }
                        } else {
                            syntax_err(self, "State has no closing token", &open_token);
                        }
                        break 'statements;
                    }

                    if let Some(next_token) = next_token {
                        match next_token.kind.clone() {
                            TokenKind::Arrow => {
                                if let Some(destination) = self.parse_destination(&next_token) {
                                    match_statements.into_iter().for_each(|match_statement| {
                                        current_state_definition.push_statement(Statement::new(
                                            destination,
                                            match_statement,
                                        ));
                                    })
                                } else {
                                    syntax_err(
                                        self,
                                        "Could find valid destination after ",
                                        &next_token,
                                    );
                                }
                            }
                            _ => {
                                syntax_err(self, "Expected '=>' here", &next_token);
                            }
                        }
                    } else {
                        syntax_err(
                            self,
                            "A token is missing after a match declaration",
                            &open_token,
                        );
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

    fn parse_left_side_inputs(&mut self) -> (Vec<StatementMatchKind>, Option<Token>) {
        let mut result = Vec::new();

        let mut buffered_match_kinds = Vec::new();

        'input_loop: while let Some(token) = self.parser.get_next_token() {
            match token.kind.clone() {
                TokenKind::Arrow => {
                    result.append(&mut buffered_match_kinds);
                    return (result, Some(token));
                }
                TokenKind::UnderScore => {
                    if !buffered_match_kinds.is_empty() {
                        syntax_err(self, "| required between match kinds", &token);
                    }
                    buffered_match_kinds.push(StatementMatchKind::Default);
                }
                TokenKind::Char(chr) => {
                    if !buffered_match_kinds.is_empty() {
                        syntax_err(self, "| required between match kinds", &token);
                    }
                    buffered_match_kinds.push(StatementMatchKind::Literal(chr));
                }
                TokenKind::CharSequence(sequence) => {
                    if !buffered_match_kinds.is_empty() {
                        syntax_err(self, "| required between match kinds", &token);
                    }
                    buffered_match_kinds.push(StatementMatchKind::Sequence(sequence));
                }
                TokenKind::Range => {
                    if buffered_match_kinds.len() > 1 {
                        syntax_err(self, "Cannot apply range on multiple literals", &token);
                    }

                    if let Some(range_open) =
                        buffered_match_kinds.get(buffered_match_kinds.len() - 1)
                    {
                        match range_open {
                            StatementMatchKind::Literal(range_open) => {
                                if let Some(range_close) = self.parser.get_next_token() {
                                    match range_close.kind.clone() {
                                        TokenKind::Char(range_close) => {
                                            let range = CharRange::new(*range_open, range_close);
                                            let _ = buffered_match_kinds.pop();
                                            buffered_match_kinds
                                                .push(StatementMatchKind::Range(range));
                                        }
                                        _ => {
                                            syntax_err(
                                                self,
                                                "Cannot close range with",
                                                &range_close,
                                            );
                                        }
                                    }
                                } else {
                                    syntax_err(self, "Expected range close after", &token);
                                }
                            }
                            _ => {
                                syntax_err(
                                    self,
                                    "Only char literals cvan be range open before",
                                    &token,
                                );
                            }
                        }
                    } else {
                        syntax_err(self, "Range needs opening literal", &token);
                    }
                }
                TokenKind::Line => {
                    result.append(&mut buffered_match_kinds);
                }
                _ => {
                    return (result, Some(token));
                }
            }
        }

        return (result, None);
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
