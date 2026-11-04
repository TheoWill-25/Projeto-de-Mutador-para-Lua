mod scanner;
mod mutador;

use std::fs;
use std::process::Command;

use scanner::start_end_finder;
use scanner::symbol_scanner;
use mutador::symbol_mutator;
use scanner::Candidate;
use mutador::Mutation;




fn main() {
    let codigo = fs::read_to_string("codigo.lua").expect("Erro ao ler o arquivo");

    let (start_indx, end_indx) = start_end_finder(&codigo);
    //println!("{} -> {}", start_indx, end_indx);

    let symbols: Vec<Candidate> = symbol_scanner(&codigo, start_indx, end_indx);
    
    /*for symbol in &symbols {
        println!("{:?}", symbol);
    }*/

    let mutation: Vec<Mutation> = symbol_mutator(&codigo, symbols);

    let survived_mutations: Vec<&Mutation> = mutation
        .iter()
        .filter(|m| m.failed_tests.is_empty())
        .collect();

    let json = serde_json::to_string_pretty(&survived_mutations)
        .expect("Erro ao gerar JSON");

    fs::write("relatorio.json", json)
        .expect("Erro ao escrever relatório");

    println!("mutações totais: {}", mutation.len());
    println!("mutações sobreviventes: {}", survived_mutations.len());

}
