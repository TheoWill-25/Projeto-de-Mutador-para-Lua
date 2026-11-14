mod scanner;
mod mutador;
mod parser;

use std::fs;

use scanner::symbol_scanner;
use mutador::symbol_mutator;
use scanner::Candidate;
use mutador::Mutation;
use parser::parse_lua;




fn main() {
    let path_code: String = "codigo.lua".to_string();
    let path_suite: String = "suit.lua".to_string();

    let codigo = fs::read_to_string(&path_code)
        .expect("Erro ao ler codigo.lua");

    let ast: full_moon::ast::Ast = parse_lua(&codigo);

    let json = serde_json::to_string_pretty(&ast)
        .expect("Erro ao converter AST para JSON");

    fs::write("AST.json", &json)
        .expect("Erro ao escrever relatório");

    let symbols: Vec<Candidate> = symbol_scanner(&ast);

    let mutation: Vec<Mutation> = symbol_mutator(&codigo, symbols, path_code, path_suite);

    let survived_mutations: Vec<&Mutation> = mutation
        .iter()
        .filter(|m| m.failed_tests.is_empty() && m.error == false)
        .collect();

    let error_mutations: Vec<&Mutation> = mutation
        .iter()
        .filter(|m| m.error == true)
        .collect();

    let json = serde_json::to_string_pretty(&survived_mutations)
        .expect("Erro ao gerar JSON");

    fs::write("relatorio.json", json)
        .expect("Erro ao escrever relatório");

    println!("mutações totais: {}", mutation.len());
    println!("mutações sobreviventes: {}", survived_mutations.len());
    println!("mutações com erro: {}", error_mutations.len());
}
