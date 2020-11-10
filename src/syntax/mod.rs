pub mod primative;

use std::vec::{Vec};
use std::cell::Cell;

use crate::types::*;
use self::primative::PrimativeParser;

pub struct SyntaxParser  {
    tokens: Box<Vec<Token>>,
    index: Cell<usize>,
}

impl SyntaxParser {
    pub fn new(tokens: Box<Vec<Token>>) -> SyntaxParser {
        SyntaxParser {
            tokens: tokens,
            index: Cell::new(0)
        }
    }

    pub fn parse(&self) {
    }

    fn peek_token(&self) -> Result<&Token, ()> {
        match self.tokens.get(self.index.get()) {
            Some(token) => Ok(token),
            None => Err(())
        }
    }

    fn previous_token(&self) -> Result<&Token, ()> {
        match self.tokens.get(self.index.get() - 1) {
            Some(token) => Ok(token),
            None => Err(())
        }
    }

    fn next_token(&self) -> Result<&Token, ()> {
        match self.tokens.get(self.index.get() + 1) {
            Some(token) => Ok(token),
            None => Err(())
        }
    }

    fn increase(&self) {
        self.index.set(self.index.get() + 1);
    }

    fn consume_token(&self) -> Option<&Token> {
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

    fn get_operator(&self, token: &Token) -> BramaOperatorType {
        return match token.token_type {
            BramaTokenType::Operator(operator) => operator,
            _ => BramaOperatorType::None
        };
    }
}