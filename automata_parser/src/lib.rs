extern crate automata;
extern crate colored;

extern crate automata_core;

pub mod tokens;

use tokens::{ScopeType::*, Token, TokenKind::*};
use automata::Automata;
use std::str::Chars;
use colored::*;
use std::collections::VecDeque;
use automata_core::string_interning::*;

/// Parses input into a series of tokens
#[derive(Debug)]
pub struct AutomataParser<'input> {
    raw: &'input str,
    input: Chars<'input>,
    buffered_input: VecDeque<char>,
    line: usize,
    column: usize,
    index: usize,
}

impl<'input> AutomataParser<'input> {
    /// Create a new AutomataParser given some input
    pub fn new(input: &'input str) -> Self {
        let input_chars = input.chars();

        AutomataParser {
            raw: input,
            input: input_chars,
            line: 0,
            column: 0,
            index: 0,
            buffered_input: VecDeque::new(),
        }
    }

    /// Goes through every token and prints it. Can be used to check input validity
    pub fn check(&mut self) {
        let automata_result = Automata::new();

        while let Some(token) = self.get_next_token() {
            println!("{:?}", token);
        }

        println!("End state: \n {:#?}", self);
    }

    /// Get the next character in the input stream
    /// This supports buffering for look ahead and takes care of whitespaces / new lines
    fn get_next_char(&mut self) -> Option<char> {
        if let Some(buffered_chr) = self.buffered_input.pop_front() {
            return Some(buffered_chr);
        }

        'input_loop: while let Some(chr) = self.input.next() {
            self.column += 1;
            self.index += 1;

            match chr {
                '\n' => {
                    self.line += 1;
                    self.column = 0;
                    self.buffered_input.push_back(' ');
                    continue 'input_loop;
                }
                _ if chr.is_whitespace() => {
                    self.column += 1;
                    self.buffered_input.push_back(' ');
                    continue 'input_loop;
                }
                _ => {
                    if let Some(buffered_chr) = self.buffered_input.pop_front() {
                        self.buffered_input.push_back(chr);
                        return Some(buffered_chr);
                    }

                    return Some(chr);
                }
            }
        }

        return None;
    }

    /// Get the column location from start to current location
    fn get_column_location_from(&self, start: usize) -> (usize, usize) {
        (start, self.column)
    }

    /// Get the line location from start to current location
    fn get_line_location_from(&self, start: usize) -> (usize, usize) {
        (start, self.line)
    }

    /// Get the index location from start to current location
    fn get_index_location_from(&self, start: usize) -> (usize, usize) {
        (start, self.index)
    }

    /// Get the next token from the input
    pub fn get_next_token(&mut self) -> Option<Token> {
        let mut chr = self.get_next_char()?;

        while chr.is_whitespace() {
            chr = self.get_next_char()?;
        }

        let column_start = self.column - 1;
        let line_start = self.line;
        let index_start = self.index - 1;

        /// A macro that returns the token, taking care of debug info
        macro_rules! return_token {
            ($kind: expr) => {
                return Some(Token::new(
                    $kind,
                    self.get_column_location_from(column_start),
                    self.get_line_location_from(line_start),
                    self.get_index_location_from(index_start),
                ));
            };
        }

        /// A macro to output parse errors
        macro_rules! parse_err {
            //TODO: Make formatting on par with the syntax errors
            ($err: expr) => {
                let error_source = &self.raw[index_start..self.index];

                let error_message = format!(
                    " starting at line: {}, Col: {}\n\tCurrent: {}",
                    line_start, column_start, error_source
                );

                eprintln!("{}{}", $err, error_message);
            };
        }

        match chr {
            //identifier
            | 'a'...'z' | 'A'...'Z' => {
                let mut identifier = String::new();
                identifier.push(chr);

                while let Some(chr) = self.get_next_char() {
                    match chr {
                        'a'...'z' | 'A'...'Z' | '0'...'9' | '_' => {
                            identifier.push(chr);
                        }
                        chr => {
                            self.buffered_input.push_front(chr);
                            return_token!(Identifier(intern(identifier)));
                        }
                    }
                }
            }
            | '0'...'9' => {
                let mut number: i32 = chr.to_digit(10).unwrap() as i32;

                while let Some(chr) = self.get_next_char() {
                    match chr {
                        '0'...'9' => {
                            number *= 10;
                            number += chr.to_digit(10).unwrap() as i32;
                        }
                        chr => {
                            self.buffered_input.push_front(chr);
                            parse_err!("Digit cannot contain letter");
                        }
                    }
                }

                return_token!(Integer(number));
            }
            //arrow
            '=' => {
                if let Some(chr) = self.get_next_char() {
                    match chr {
                        '>' => {
                            return_token!(Arrow);
                        }
                        _ => {
                            parse_err!("Could not parse arrow");
                            return None;
                        }
                    }
                } else {
                    parse_err!("Error short circuit");
                    return None;
                }
            }
            //Scope start
            ':' => {
                return_token!(Column);
            }
            //Range
            '.' => {
                if let Some(chr) = self.get_next_char() {
                    match chr {
                        '.' => {
                            return_token!(Range);
                        }
                        _ => {
                            parse_err!("Could not parse range");
                            return None;
                        }
                    }
                }
            }
            // Semi column
            ';' => {
                return_token!(SemiColumn);
            }
            // Character literal
            '\'' => {
                if let Some(middle_char) = self.get_next_char() {
                    if let Some(chr) = self.get_next_char() {
                        match chr {
                            '\'' => {
                                return_token!(Char(middle_char));
                            }
                            _ => {
                                parse_err!("Could not parse char literal");
                                return None;
                            }
                        }
                    }
                }
            }
            // Scopes
            '{' => {
                return_token!(Scope(Open));
            }
            '}' => {
                return_token!(Scope(Close));
            }
            //UnderScore
            '_' => {
                return_token!(UnderScore);
            }
            // Unknown
            chr => {
                parse_err!(format!("No pattern for {} for ", chr));
            }
        }

        None
    }
}
