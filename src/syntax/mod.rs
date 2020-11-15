pub mod primative;
pub mod unary;
pub mod util;
pub mod binary;
pub mod control;

use std::vec::{Vec};
use std::cell::Cell;

use crate::types::*;
use crate::types::SyntaxParser;
use self::control::ExpressionParser;

impl SyntaxParser {
    pub fn new(tokens: Box<Vec<Token>>) -> SyntaxParser {
        SyntaxParser {
            tokens: tokens,
            index: Cell::new(0)
        }
    }

    pub fn parse(&self) -> AstResult {
        return ExpressionParser::parse(&self);
    }

    fn peek_token(&self) -> Result<&Token, ()> {
        match self.tokens.get(self.index.get()) {
            Some(token) => Ok(token),
            None => Err(())
        }
    }

    #[allow(dead_code)]
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
    
    #[allow(dead_code)]
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

    fn check_keyword(&self, keyword: &BramaKeywordType) -> bool {
        let token = self.peek_token();
        if token.is_err() { return false; }
        return match token.unwrap().token_type {
            BramaTokenType::Keyword(token_keyword) => *keyword == token_keyword,
            _ => false
        }
    }

    fn match_keyword(&self, keywords: &[BramaKeywordType]) -> Option<BramaKeywordType> {
        for keyword in keywords {
            if self.check_keyword(keyword) {
                self.consume_token();
                return Some(*keyword);
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

    #[allow(dead_code)]
    fn get_operator(&self, token: &Token) -> BramaOperatorType {
        return match token.token_type {
            BramaTokenType::Operator(operator) => operator,
            _ => BramaOperatorType::None
        };
    }
}