mod scanner;

use std::fs;
use std::process::Command;

use scanner::start_end_finder;
use scanner::symbol_scanner;
use scanner::Candidate;



fn main() {
    let mut codigo = fs::read_to_string("codigo.lua").expect("Erro ao ler o arquivo");

    let (start_indx, end_indx) = start_end_finder(&codigo);
    println!("{} -> {}", start_indx, end_indx);

    let symbols: Vec<Candidate> = symbol_scanner(&codigo, start_indx, end_indx);
    
    for symbol in &symbols {
        println!("{:?}", symbol);
    }



}
