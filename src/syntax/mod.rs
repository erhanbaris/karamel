use std::vec::Vec;
use crate::types::*;

pub struct SyntaxParser<'a>  {
    tokens: &'a Vec<Token>
}

impl<'a> SyntaxParser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> SyntaxParser {
        SyntaxParser {
            tokens: tokens
        }
    }

    pub fn parse(&mut self) {
    }


    fn primary_expr(&mut self) {

    }
}