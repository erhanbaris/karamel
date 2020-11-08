use std::vec::{Vec};
use std::str::Chars;
use std::iter::Peekable;
use std::slice::Iter;
use std::cell::Cell;


use crate::types::*;

pub struct SyntaxParser<'a>  {
    tokens: &'a Vec<Token>,
    index: Cell<usize>,

}

impl<'a> SyntaxParser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> SyntaxParser {
        SyntaxParser {
            tokens: tokens,
            index: Cell::new(0)
        }
    }

    pub fn parse(&self) {
    }

    pub fn is_primative(&self) -> bool {
        return match self.peek_ast() {
            None => false,
            Some(token) => {
                match &token.token_type {
                    BramaTokenType::Integer(num) => true,
                    BramaTokenType::Double(num) => true,
                    BramaTokenType::Text(s) => true,
                    BramaTokenType::Keyword(keyword) => {
                        *keyword == BramaKeywordType::True || *keyword == BramaKeywordType::False
                    },
                    BramaTokenType::WhiteSpace(c) => false,
                    BramaTokenType::NewLine(c) => false,
                    BramaTokenType::Symbol(sym) => false,
                    BramaTokenType::Operator(opt) => {
                        match *opt {
                            BramaOperatorType::ColonMark => {
                                match self.next_ast() {
                                    Some(token) => {
                                        match &token.token_type {
                                            BramaTokenType::Symbol(symbol) => {
                                                true
                                            },
                                            _ => false
                                        }
                                    }
                                    None => false
                                }
                            },
                            _ => false
                        }
                    },
                    BramaTokenType::None => false
                }
            }
        };
    }

    fn peek_ast(&self) -> Option<&Token> {
        self.tokens.get(self.index.get())
    }

    fn previous_ast(&self) -> Option<&Token> {
        self.tokens.get(self.index.get() - 1)
    }

    fn next_ast(&self) -> Option<&Token> {
        self.tokens.get(self.index.get() + 1)
    }

    fn increase(&self) {
        self.index.set(self.index.get() + 1);
    }

    fn consume_ast(&self) -> Option<&Token> {
        self.index.set(self.index.get() + 1);
        self.tokens.get(self.index.get())
    }

    pub fn primary_expr(&self) -> AstResult {
        if self.is_primative() {
            if let Some(token) = self.peek_ast() {
                return self.create_primative_ast(token);
            }
        }

        Err((String::from("not implemented"), 0, 0))
    }

    fn create_primative_ast(&self, token :&Token) -> AstResult {
        let ast = match &token.token_type {
            BramaTokenType::Integer(int)      => BramaAstType::Primative(BramaPrimative::Integer(*int)),
            BramaTokenType::Double(double)    => BramaAstType::Primative(BramaPrimative::Double(*double)),
            BramaTokenType::Text(text)        => BramaAstType::Primative(BramaPrimative::String(text.to_string())),
            BramaTokenType::Keyword(keyword)  => {
                match keyword {
                    BramaKeywordType::True  => BramaAstType::Primative(BramaPrimative::Bool(true)),
                    BramaKeywordType::False => BramaAstType::Primative(BramaPrimative::Bool(false)),
                    _ => BramaAstType::None
                }
            },
            BramaTokenType::Operator(operator) => {
                if *operator == BramaOperatorType::ColonMark {
                    if let Some(next_token) = self.next_ast() {
                        match &next_token.token_type {
                            BramaTokenType::Symbol(symbol) => {
                                self.consume_ast();
                                return Ok(BramaAstType::Primative(BramaPrimative::Atom(symbol.to_string())));
                            },
                            _ => {
                                return Err((String::from("Token not recognized"), token.line, token.column));
                            }
                        }

                    }
                }

                BramaAstType::None
            },
            _ => BramaAstType::None
        };

        if ast == BramaAstType::None {
            return Err((String::from("Token not recognized"), token.line, token.column));
        }

        return Ok(ast);
    }
}