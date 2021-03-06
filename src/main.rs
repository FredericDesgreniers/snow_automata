extern crate automata;
extern crate automata_parser;
extern crate automata_syntax;

use automata_syntax::SyntaxParser;
use automata::Automata;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("snow_automata v: {}", env!("CARGO_PKG_VERSION"));

    let mut automata_text = String::new();

    let mut automata_file = File::open("automata.sa").expect("Could not open input file...");

    let _ = automata_file
        .read_to_string(&mut automata_text)
        .expect("Could not read file...");

    let mut parser = SyntaxParser::new(&automata_text);
    let state_definitions = parser.parse();

    let automata = Automata::resolve_from(state_definitions);

    println!("{:#?}", automata);
}
