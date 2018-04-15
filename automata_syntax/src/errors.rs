#![macro_use]

pub use colored::*;
use super::SyntaxParser;
use automata_parser::tokens::Token;

pub fn syntax_err<T: AsRef<str>>(syntax_parser: &SyntaxParser, message: T, token: &Token) {
    let debug_info = token.debug_info;

    let (token_start, token_end) = debug_info.index_location;

    let source_start = if token_start > 5 { token_start - 5 } else { 0 };

    let source_end = if token_end < syntax_parser.input.len() - 5 {
        token_end + 5
    } else {
        syntax_parser.input.len()
    };

    let err_source_before = syntax_parser.input[source_start..token_start].trim();
    let err_source_after = syntax_parser.input[token_end..source_end].trim();

    let err_source = syntax_parser.input[token_start..token_end].trim();

    eprintln!("{}", message.as_ref().bright_red().underline());
    eprintln!("Token:\t{}", format!("{:?}", token.kind).bright_cyan());
    eprintln!(
        "Source:\t{}{}{}",
        err_source_before.yellow(),
        err_source.yellow().underline(),
        err_source_after.yellow()
    );
    eprintln!(
        "At {}: {} and {}: {}\n",
        "line".green(),
        debug_info.line_location.0,
        "column".green(),
        debug_info.column_location.0
    );
}
