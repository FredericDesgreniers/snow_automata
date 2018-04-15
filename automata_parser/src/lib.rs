extern crate automata;
extern crate colored;

pub mod tokens;

use tokens::{TokenKind::*, ScopeType::*, Token};
use automata::Automata;
use std::str::Chars;
use colored::*;

#[derive(Debug)]
pub struct AutomataParser<'input> {
    raw: &'input str,
    input: Chars<'input>,
    buffered_input: Option<char>,
    line: usize,
    column: usize,
    index: usize
}

impl<'input> AutomataParser<'input> {
    pub fn new(input: &'input str) -> Self {
        let input_chars = input.chars();

        AutomataParser {
            raw: input,
            input: input_chars,
            line: 0,
            column: 0,
            index: 0,
            buffered_input: None
        }
    }

    pub fn check(&mut self) -> Automata{

        let automata_result = Automata::new();

        while let Some(token) = self.get_next_token() {
            println!("{:?}", token);
        }

        println!("End state: \n {:#?}", self);

        automata_result
    }

    fn get_next_char(&mut self) -> Option<char> {

        if let Some(buffered_chr) = self.buffered_input {
            self.buffered_input = None;
            return Some(buffered_chr);
        }

        'input_loop: while let Some(chr) = self.input.next(){
            self.column += 1;
            self.index += 1;

            match chr {
                '\n' => {
                    self.line += 1;
                    self.column = 0;
                    self.buffered_input = Some(' ');
                    continue 'input_loop;
                },
                _ if chr.is_whitespace() => {
                    self.column += 1;
                    self.buffered_input = Some(' ');
                    continue 'input_loop;
                },
                _ => {
                    if let Some(buffered_chr) = self.buffered_input {
                        self.buffered_input = Some(chr);
                        return Some(buffered_chr);
                    }

                    return Some(chr);
                }
            }
        }

        return None;
    }

    fn get_column_location_from(&self, start: usize) -> (usize, usize) {
        (start, self.column)
    }

    fn get_line_location_from(&self, start: usize) -> (usize, usize) {
        (start, self.line)
    }

    fn get_index_location_from(&self, start: usize) -> (usize, usize) {
        (start, self.index)
    }

    pub fn get_next_token(&mut self) -> Option<Token> {

        let mut chr = self.get_next_char()?;

        while chr.is_whitespace() {
            chr = self.get_next_char()?;
        }

        let column_start = self.column-1;
        let line_start = self.line;
        let index_start = self.index-1;

        macro_rules! return_token {
            ($kind: expr) => {
                return Some(Token::new($kind,
                self.get_column_location_from(column_start),
                self.get_line_location_from(line_start),
                self.get_index_location_from(index_start)))
            };
        }

        macro_rules! parse_err {
            ($err: expr) => {
                let error_source = &self.raw[index_start..self.index];

                let error_message = format!(" starting at line: {}, Col: {}\n\tCurrent: {}", line_start, column_start, error_source);

                eprintln!("{}{}", $err, error_message);
            };
        }


        match chr {
            //identifier
            | 'a'...'z'
            | 'A'...'Z' => {
                let mut identifier = String::new();
                identifier.push(chr);

                while let Some(chr) = self.get_next_char() {
                    match chr {
                        'a'...'z'
                        | 'A'...'Z'
                        | '0'...'9'
                        | '_' => {
                            identifier.push(chr);
                        },
                        chr => {
                            self.buffered_input = Some(chr);
                            return_token!(Identifier(identifier));
                        }
                    }
                }
            },
            | '0'...'9' => {
                let mut number:i32 = chr.to_digit(10).unwrap() as i32;

                while let Some(chr) = self.get_next_char() {
                    match chr {
                        '0'...'9' => {
                            number *= 10;
                            number += chr.to_digit(10).unwrap() as i32;
                        },
                        chr => {
                            self.buffered_input = Some(chr);
                            parse_err!("Digit cannot contain letter");
                        }
                    }
                }

                return_token!(Integer(number));
            },
            //arrow
            '=' => {
                if let Some(chr) = self.get_next_char() {
                    match chr {
                        '>' => {
                            return_token!(Arrow);
                        },
                        _ => {
                            parse_err!("Could not parse arrow");
                            return None;
                        }
                    }
                } else {
                    parse_err!("Error short circuit");
                    return None;
                }
            },
            //Scope start
            ':' => {
                return_token!(Column);
            },
            //Range
            '.' => {
                if let Some(chr) = self.get_next_char() {
                    match chr {
                        '.' => {
                            return_token!(Range);
                        },
                        _ => {
                            parse_err!("Could not parse range");
                            return None;
                        }
                    }
                }
            },
            ';' => {
                return_token!(SemiColumn);
            },
            '\'' => {
                if let Some(middle_char) = self.get_next_char() {
                    if let Some(chr) = self.get_next_char() {
                        match chr {
                            '\'' => {
                                return_token!(Char(middle_char));
                            },
                            _ => {
                                parse_err!("Could not parse char literal");
                                return None;
                            }
                        }
                    }
                }
            },
            '{' => {
                return_token!(Scope(Open));
            },
            '}' => {
                return_token!(Scope(Close));
            }
            chr => {
                parse_err!(format!("No pattern for {} for ", chr));
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_token_sequence() {
        let input_text = "var1: \
        'a'..'b' => \
        ";
    }
}
