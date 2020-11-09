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
        return match self.peek_token() {
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

    fn peek_token(&self) -> Option<&Token> {
        self.tokens.get(self.index.get())
    }

    fn previous_token(&self) -> Option<&Token> {
        self.tokens.get(self.index.get() - 1)
    }

    fn next_ast(&self) -> Option<&Token> {
        self.tokens.get(self.index.get() + 1)
    }

    fn increase(&self) {
        self.index.set(self.index.get() + 1);
    }

    fn consume_token(&self) -> Option<&Token> {
        self.index.set(self.index.get() + 1);
        self.tokens.get(self.index.get())
    }

    pub fn primary_expr(&self) -> AstResult {
        if self.is_primative() {
            return match self.peek_token() {
                Some(token) => self.create_primative_ast(token),
                None => Err(("", 0, 0))
            }
        }

        if self.match_operator(&[BramaOperatorType::SquareBracketStart]).is_some() {
            let mut ast_vec   = Vec::new();
            if self.match_operator(&[BramaOperatorType::SquareBracketEnd]).is_none() {

                loop {
                    if let Ok(ast) = self.create_primative_ast(self.peek_token().unwrap()) {
                        ast_vec.push(Box::new(ast));
                        self.consume_token();
                    }

                    if self.match_operator(&[BramaOperatorType::Comma]).is_some()  {
                        continue;
                    }
                    else if self.check_operator(&BramaOperatorType::SquareBracketEnd) {
                        break;
                    }
                    else {
                        return Err(("Array not valid", 0, 0));
                    }
                }
            }

            return Ok(BramaAstType::Primative(BramaPrimative::List(ast_vec)));
        }

        Err(("not implemented", 0, 0))
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
                                self.consume_token();
                                return Ok(BramaAstType::Primative(BramaPrimative::Atom(symbol.to_string())));
                            },
                            _ => {
                                return Err(("Token not recognized", token.line, token.column));
                            }
                        }

                    }
                }

                BramaAstType::None
            },
            _ => BramaAstType::None
        };

        if ast == BramaAstType::None {
            return Err(("Token not recognized", token.line, token.column));
        }

        return Ok(ast);
    }

    fn check_operator(&self, operator: &BramaOperatorType) -> bool {
        let token = self.peek_token();
        if token.is_none() { return false; }
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

    fn get_operator(&self, token: &Token) -> BramaOperatorType {
        return match token.token_type {
            BramaTokenType::Operator(operator) => operator,
            _ => BramaOperatorType::None
        };
    }

    pub fn create_unary(&self) -> AstResult {
        if let Some(operator) = self.match_operator(&[BramaOperatorType::Addition,
            BramaOperatorType::Subtraction, 
            BramaOperatorType::Deccrement, 
            BramaOperatorType::Not, 
            BramaOperatorType::BitwiseNot]) {

            let mut unary_ast = BramaAstType::None;
            let token         = &self.peek_token().unwrap();

            match operator {
                /* +1024 -1024 */
                BramaOperatorType::Addition | BramaOperatorType::Subtraction => {
                    if token.token_type.is_integer() || token.token_type.is_double() {
                        if let Ok(ast) = self.create_primative_ast(&token) {
                            unary_ast = ast;
                        }
                    }
                },

                /* ! */
                BramaOperatorType::Not => {
                    if token.token_type.is_operator() || token.token_type.is_double() {
                        if let Ok(ast) = self.create_primative_ast(&token) {
                            unary_ast = ast;
                        }
                    }
                },
                _ => ()
            }
            return Ok(BramaAstType::PrefixUnary(Box::new(unary_ast)));
        }

        return Err(("Invalid unary operation", 0, 0))
    }
}