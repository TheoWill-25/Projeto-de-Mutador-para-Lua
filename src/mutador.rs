
use crate::scanner::Candidate;
use std::fs;
use serde::Serialize;
use std::process::Command;

#[derive(Debug, Serialize, Clone)]
pub struct Mutation {
    pub candidate: Candidate,
    pub replacement: String,
    pub og_line: String,
    pub mod_line : String,
    pub num_tests: i32,
    pub failed_tests: Vec<String>,
    
}

pub fn symbol_mutator(code: &str, symbol: Vec<Candidate>) -> Vec<Mutation> {
    let mut mutation: Vec<Mutation> = Vec::new();

    

    for sym in symbol{
        let line_offset: usize = code
            .lines()
            .take(sym.line - 1)
            .map(|line| line.len() + 1)
            .sum();

        let line = code.lines()
            .nth(sym.line - 1)
            .unwrap();

        let start_byte = line.char_indices()
            .nth(sym.start)
            .map(|(i, _)| i)
            .unwrap_or(line.len());

        let end_byte = line.char_indices()
            .nth(sym.end)
            .map(|(i, _)| i)
            .unwrap_or(line.len());


        let new_symbols = symbol_list(&sym);

        for ns in new_symbols {
            let mut codigo_mutado = code.to_string();

            codigo_mutado.replace_range(
                line_offset + start_byte..line_offset + end_byte,
                &ns
            );

            let linha_modificada = codigo_mutado.lines()
                .nth(sym.line - 1)
                .unwrap();

            fs::write("codigo.lua", &codigo_mutado).expect("Erro ao escrever arquivo");

            let output = Command::new("lua")
                        .arg("suit.lua")
                        .output()
                        .expect("Erro ao executar Lua");
            
            let stdout = String::from_utf8_lossy(&output.stdout);

            let mut tests: Vec<String> = Vec::new();
            let mut num: i32 = 0;

            for linha in stdout.lines() {
                if linha.contains("[PASS]") {
                    num += 1;
                }
                if linha.contains("[FAIL]") {
                    let nome = linha.split("[FAIL] ").nth(1).unwrap().trim();
                    tests.push(nome.to_string());
                    num += 1;
                }
            }

            mutation.push( Mutation { candidate: sym.clone(), replacement: ns, failed_tests: tests, num_tests: num,
                                        og_line: line.to_string(), mod_line: linha_modificada.to_string()});
        }

        

    }

    fs::write("codigo.lua", code).expect("Erro ao escrever arquivo");

    mutation
}


fn symbol_list(sym: &Candidate) -> Vec<String> {
    let mut new_symbs: Vec<String> = Vec::new();

    match sym.original.as_str() {
        "<" => {
            new_symbs.push(">".to_string());
            new_symbs.push("<=".to_string());
            new_symbs.push("==".to_string());
        }
        ">" => {
            new_symbs.push("<".to_string());
            new_symbs.push(">=".to_string());
            new_symbs.push("==".to_string());
        }
        "~=" =>{
            new_symbs.push("==".to_string());
        }
        "~" =>{
            new_symbs.push("".to_string());
        }
        "<=" =>{
            new_symbs.push("<".to_string());
            new_symbs.push(">".to_string());
            new_symbs.push("==".to_string());
        }
        ">=" =>{
            new_symbs.push("<".to_string());
            new_symbs.push(">".to_string());
            new_symbs.push("==".to_string());
        }
        "==" =>{
            new_symbs.push("<".to_string());
            new_symbs.push(">".to_string());
            new_symbs.push("<=".to_string());
            new_symbs.push(">=".to_string());
            new_symbs.push("~=".to_string());
        }
        _ => new_symbs.push(sym.original.to_string()),
        
    }

    new_symbs
}