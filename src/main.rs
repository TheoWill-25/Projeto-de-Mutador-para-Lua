mod scanner;
mod mutador;
mod parser;

use std::fs;
use std::process::Command;

use scanner::start_end_finder;
use scanner::symbol_scanner;
use mutador::symbol_mutator;
use scanner::Candidate;
use mutador::Mutation;
use parser::parse_lua;




fn main() {
    parse_lua();
}
