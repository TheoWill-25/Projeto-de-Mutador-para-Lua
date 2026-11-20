use serde::Serialize;
use full_moon::node::Node;
use full_moon::visitors::Visitor;
use full_moon::ast::{Expression, BinOp, UnOp};

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

                BinOp::Plus(token) => {

                    self.candidates.push(Candidate {
                        kind: "ArithmeticOperator".to_string(),
                        original: "+".to_string(),
                        start: token.token().start_position().bytes(),
                        end: token.token().end_position().bytes(),
                        line: token.token().start_position().line(),
                    });
                }

                BinOp::Minus(token) => {

                    self.candidates.push(Candidate {
                        kind: "ArithmeticOperator".to_string(),
                        original: "-".to_string(),
                        start: token.token().start_position().bytes(),
                        end: token.token().end_position().bytes(),
                        line: token.token().start_position().line(),
                    });
                }

                BinOp::Star(token) => {

                    self.candidates.push(Candidate {
                        kind: "ArithmeticOperator".to_string(),
                        original: "*".to_string(),
                        start: token.token().start_position().bytes(),
                        end: token.token().end_position().bytes(),
                        line: token.token().start_position().line(),
                    });
                }

                BinOp::Slash(token) => {

                    self.candidates.push(Candidate {
                        kind: "ArithmeticOperator".to_string(),
                        original: "/".to_string(),
                        start: token.token().start_position().bytes(),
                        end: token.token().end_position().bytes(),
                        line: token.token().start_position().line(),
                    });
                }

                BinOp::DoubleSlash(token) => {

                    self.candidates.push(Candidate {
                        kind: "ArithmeticOperator".to_string(),
                        original: "//".to_string(),
                        start: token.token().start_position().bytes(),
                        end: token.token().end_position().bytes(),
                        line: token.token().start_position().line(),
                    });
                }

                BinOp::Percent(token) => {

                    self.candidates.push(Candidate {
                        kind: "ArithmeticOperator".to_string(),
                        original: "%".to_string(),
                        start: token.token().start_position().bytes(),
                        end: token.token().end_position().bytes(),
                        line: token.token().start_position().line(),
                    });
                }

                BinOp::Caret(token) => {

                    self.candidates.push(Candidate {
                        kind: "ArithmeticOperator".to_string(),
                        original: "^".to_string(),
                        start: token.token().start_position().bytes(),
                        end: token.token().end_position().bytes(),
                        line: token.token().start_position().line(),
                    });
                }

                BinOp::Ampersand(token ) => {

                    self.candidates.push(Candidate {
                        kind: "BitwiseOperator".to_string(),
                        original: "&".to_string(),
                        start: token.token().start_position().bytes(),
                        end: token.token().end_position().bytes(),
                        line: token.token().start_position().line(),
                    });
                }

                BinOp::Pipe(token ) => {

                    self.candidates.push(Candidate {
                        kind: "BitwiseOperator".to_string(),
                        original: "|".to_string(),
                        start: token.token().start_position().bytes(),
                        end: token.token().end_position().bytes(),
                        line: token.token().start_position().line(),
                    });
                }

                BinOp::Tilde(token ) => {

                    self.candidates.push(Candidate {
                        kind: "BitwiseOperator".to_string(),
                        original: "~".to_string(),
                        start: token.token().start_position().bytes(),
                        end: token.token().end_position().bytes(),
                        line: token.token().start_position().line(),
                    });
                }

                BinOp::DoubleLessThan(token ) => {

                    self.candidates.push(Candidate {
                        kind: "BitwiseOperator".to_string(),
                        original: "<<".to_string(),
                        start: token.token().start_position().bytes(),
                        end: token.token().end_position().bytes(),
                        line: token.token().start_position().line(),
                    });
                }

                BinOp::DoubleGreaterThan(token ) => {

                    self.candidates.push(Candidate {
                        kind: "BitwiseOperator".to_string(),
                        original: ">>".to_string(),
                        start: token.token().start_position().bytes(),
                        end: token.token().end_position().bytes(),
                        line: token.token().start_position().line(),
                    });
                }

                BinOp::TwoDots(token) => {
                    self.candidates.push(Candidate{
                        kind: "ConcatenationOperator".to_string(),
                        original: "..".to_string(),
                        start: token.token().start_position().bytes(),
                        end: token.token().end_position().bytes(),
                        line: token.token().start_position().line(),
                    });
                }

                _=> {}

            }
            
        }

        if let Expression::UnaryOperator{ unop, .. } = expr {
            match unop {
                UnOp::Not(token) => {

                    self.candidates.push(Candidate {
                        kind: "LogicOperator".to_string(),
                        original: "not".to_string(),
                        start: token.token().start_position().bytes(),
                        end: token.token().end_position().bytes(),
                        line: token.token().start_position().line(),
                    });
                }

                UnOp::Tilde(token) => {

                    self.candidates.push(Candidate {
                        kind: "UnaryBitwiseOperator".to_string(),
                        original: "~".to_string(),
                        start: token.token().start_position().bytes(),
                        end: token.token().end_position().bytes(),
                        line: token.token().start_position().line(),
                    });
                }

                UnOp::Minus(token) => {

                    self.candidates.push(Candidate {
                        kind: "UnaryArithmeticOperator".to_string(),
                        original: "-".to_string(),
                        start: token.token().start_position().bytes(),
                        end: token.token().end_position().bytes(),
                        line: token.token().start_position().line(),
                    });
                }

                UnOp::Hash(_) => {
                    if let Some((inicio, fim)) = expr.range() {
                        self.candidates.push(Candidate {
                            kind: "LengthOperator".to_string(),
                            original: "#".to_string(),
                            start: inicio.bytes(),
                            end: fim.bytes(),
                            line: inicio.line(),
                        });
                    }
                }

                _=> {}
            }
        }

        if let Expression::Number(token) = expr {

            self.candidates.push(Candidate {
                kind: "NumberValue".to_string(),
                original: token.token().to_string(),
                start: token.token().start_position().bytes(),
                end: token.token().end_position().bytes(),
                line: token.token().start_position().line(),
            });
        }
        
    }
}

pub fn symbol_scanner(ast: &full_moon::ast::Ast ) -> Vec<Candidate> {
    let mut scanner: Scanner = Scanner::default();
    
    scanner.visit_ast(ast);

    scanner.candidates
}




