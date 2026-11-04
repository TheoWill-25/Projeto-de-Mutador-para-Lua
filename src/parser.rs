use full_moon::parse;
use std::fs;

pub fn parse_lua() {
    let codigo = fs::read_to_string("test.lua")
        .expect("Erro ao ler codigo.lua");

    let ast = parse(&codigo)
        .expect("Erro ao interpretar código Lua");

    let json = serde_json::to_string_pretty(&ast)
        .expect("Erro ao converter AST para JSON");

    fs::write("AST.json", json)
        .expect("Erro ao escrever relatório");

    //println!("{:#?}", ast);
}