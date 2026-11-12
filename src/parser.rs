use full_moon::parse;

pub fn parse_lua(code: &str) -> full_moon::ast::Ast {

    let ast: full_moon::ast::Ast = parse(code)
        .expect("Erro ao interpretar código Lua");

    ast
}