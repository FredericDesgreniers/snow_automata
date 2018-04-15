extern crate automata_parser;
extern crate colored;

mod errors;

use errors::*;

use colored::*;
use automata_parser::tokens::*;


pub struct SyntaxParser<'input> {
    parser: automata_parser::AutomataParser<'input>,
    input: &'input str,
    state_definitions: Vec<StateDefinition>
}

pub struct StateDefinition {

}

impl<'input> SyntaxParser<'input> {
    pub fn new(input: &'input str) -> Self {
        println!("{}", input);
        Self {
            parser: automata_parser::AutomataParser::new(input),
            input,
            state_definitions: Vec::new()
        }
    }

    pub fn parse(&mut self) {
        while let Some(token) = self.parser.get_next_token() {
            match token.kind.clone() {
                TokenKind::Identifier(identifier) => {
                    let open_token = self.parser.get_next_token();
                    if let Some(open_token) = open_token {
                        if let Token{kind: TokenKind::Scope(ScopeType::Open), ..} = open_token {

                        } else {
                            syntax_err!(self, "Expected an open token", open_token);
                        }
                    } else {
                        syntax_err!(self, "Expected an open token after", token);
                    }


                },
                _ => {
                    syntax_err!(self, "Did not expect at start of definition", token);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
