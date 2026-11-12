
use serde::Serialize;
use full_moon::visitors::Visitor;
use full_moon::ast::{Expression, BinOp};

#[derive(Debug, Serialize, Clone)]
pub struct Candidate {
    pub line : usize,
    pub start: usize,
    pub end: usize,
    pub original: String,
    pub kind: String,
}

#[derive(Default)]
pub struct Scanner{
    candidates: Vec<Candidate>,
}

impl Visitor for Scanner {
    fn visit_expression(&mut self, expr: &Expression){
        if let Expression::BinaryOperator{ binop, .. } = expr {
            match binop {
                BinOp::GreaterThan(token) => {

                    self.candidates.push(Candidate {
                        kind: "ComparisonOperator".to_string(),
                        original: ">".to_string(),
                        start: token.token().start_position().bytes(),
                        end: token.token().end_position().bytes(),
                        line: token.token().start_position().line(),
                    });
                }

                BinOp::LessThan(token) => {

                    self.candidates.push(Candidate {
                        kind: "ComparisonOperator".to_string(),
                        original: "<".to_string(),
                        start: token.token().start_position().bytes(),
                        end: token.token().end_position().bytes(),
                        line: token.token().start_position().line(),
                    });
                }

                BinOp::TwoEqual(token) => {

                    self.candidates.push(Candidate {
                        kind: "ComparisonOperator".to_string(),
                        original: "==".to_string(),
                        start: token.token().start_position().bytes(),
                        end: token.token().end_position().bytes(),
                        line: token.token().start_position().line(),
                    });
                }

                BinOp::TildeEqual(token) => {

                    self.candidates.push(Candidate {
                        kind: "ComparisonOperator".to_string(),
                        original: "~=".to_string(),
                        start: token.token().start_position().bytes(),
                        end: token.token().end_position().bytes(),
                        line: token.token().start_position().line(),
                    });
                }

                BinOp::GreaterThanEqual(token) => {

                    self.candidates.push(Candidate {
                        kind: "ComparisonOperator".to_string(),
                        original: ">=".to_string(),
                        start: token.token().start_position().bytes(),
                        end: token.token().end_position().bytes(),
                        line: token.token().start_position().line(),
                    });
                }

                BinOp::LessThanEqual(token) => {

                    self.candidates.push(Candidate {
                        kind: "ComparisonOperator".to_string(),
                        original: "<=".to_string(),
                        start: token.token().start_position().bytes(),
                        end: token.token().end_position().bytes(),
                        line: token.token().start_position().line(),
                    });
                }

                BinOp::And(token) => {

                    self.candidates.push(Candidate {
                        kind: "LogicOperator".to_string(),
                        original: "and".to_string(),
                        start: token.token().start_position().bytes(),
                        end: token.token().end_position().bytes(),
                        line: token.token().start_position().line(),
                    });
                }

                BinOp::Or(token) => {

                    self.candidates.push(Candidate {
                        kind: "LogicOperator".to_string(),
                        original: "or".to_string(),
                        start: token.token().start_position().bytes(),
                        end: token.token().end_position().bytes(),
                        line: token.token().start_position().line(),
                    });
                }

                _=> {}
            }
        }
    }
}

pub fn symbol_scanner(ast: &full_moon::ast::Ast ) -> Vec<Candidate> {
    let mut scanner: Scanner = Scanner::default();
    
    scanner.visit_ast(ast);

    scanner.candidates
}




