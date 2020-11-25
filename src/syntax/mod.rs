pub mod primative;
pub mod unary;
pub mod util;
pub mod binary;
pub mod control;
pub mod block;
pub mod assignment;
pub mod func_call;

use std::vec::Vec;
use std::cell::Cell;

use crate::types::*;
use self::block::BlockParser;

pub type ParseType = fn(parser: &SyntaxParser) -> AstResult;

pub struct SyntaxParser {
    pub tokens: Box<Vec<Token>>,
    pub index: Cell<usize>,
    pub backup_index: Cell<usize>
}

pub trait SyntaxParserTrait {
    fn parse(parser: &SyntaxParser) -> AstResult;
}

impl SyntaxParser {
    pub fn new(tokens: Box<Vec<Token>>) -> SyntaxParser {
        SyntaxParser {
            tokens: tokens,
            index: Cell::new(0),
            backup_index: Cell::new(0)
        }
    }

    pub fn parse(&self) -> AstResult {
        return match BlockParser::parse(&self) {
            Ok(ast) => {
                if let Ok(token) = self.next_token() {
                    println!("{:?}", token);
                    return Err(("Syntax error", token.line, token.column));
                }
                Ok(ast)
            },
            other => other
        };
    }

    pub fn backup(&self) {
        self.backup_index.set(self.index.get());
    }

    pub fn restore(&self) {
        self.index.set(self.backup_index.get());
    }

    pub fn peek_token(&self) -> Result<&Token, ()> {
        match self.tokens.get(self.index.get()) {
            Some(token) => Ok(token),
            None => Err(())
        }
    }

    pub fn next_token(&self) -> Result<&Token, ()> {
        match self.tokens.get(self.index.get() + 1) {
            Some(token) => Ok(token),
            None => Err(())
        }
    }
    
    pub fn consume_token(&self) -> Option<&Token> {
        self.index.set(self.index.get() + 1);
        self.tokens.get(self.index.get())
    }

    fn check_operator(&self, operator: &BramaOperatorType) -> bool {
        let token = self.peek_token();
        if token.is_err() { return false; }
        return match token.unwrap().token_type {
            BramaTokenType::Operator(token_operator) => *operator == token_operator,
            _ => false
        }
    }

    fn match_operator(&self, operators: &[BramaOperatorType]) -> Option<BramaOperatorType> {
        for operator in operators {
            if self.check_operator(operator) {
                self.consume_token();
                return Some(*operator);
            }
        }

        return None;
    }

    fn clear_whitespaces(&self) {
        loop {
            if let Ok(current_token) = self.peek_token() {
                let done = match current_token.token_type {
                    BramaTokenType::WhiteSpace(_) => false,
                    _ => true
                };

                if done {
                    break;
                }

                self.consume_token();
            }
            else {
                break;
            }
        }
    }

    pub fn consume_newline(&self) {
        loop {
            if let Ok(current_token) = self.peek_token() {
                let done = match current_token.token_type {
                    BramaTokenType::NewLine(_) => false,
                    _ => true
                };

                if done {
                    break;
                }

                self.consume_token();
            }
            else {
                break;
            }
        }
    }
}