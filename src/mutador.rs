
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

        let new_symbols = symbol_list(&sym);

        for ns in new_symbols {
            let mut codigo_mutado = code.to_string();

            codigo_mutado.replace_range(
                sym.start..sym.end,
                &ns
            );

            let linha_modificada = codigo_mutado.lines()
                .nth(sym.line - 1)
                .unwrap();

            fs::write("codigo.lua", &codigo_mutado).expect("Erro ao escrever arquivo");

            let output = Command::new("lua")
                        .arg("suit2.lua")
                        .output()
                        .expect("Erro ao executar Lua");
            
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            if !output.status.success() {
                eprintln!("Erro ao executar os testes:");
                eprintln!("{}", stderr);
            }

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
                                        og_line: sym.line.to_string(), mod_line: linha_modificada.to_string()});
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
        "and" => {
            new_symbs.push("or".to_string());
        }
        "or" => {
            new_symbs.push("and".to_string());
        }
        _ => new_symbs.push(sym.original.to_string()),
        
    }

    new_symbs
}