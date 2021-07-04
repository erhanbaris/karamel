mod number;
mod text;
mod operator;
mod symbol;
mod line;
mod whitespace;
mod comment;

use std::str;
use std::collections::HashMap;

use crate::{error::KaramelError, types::*};
use self::number::NumberParser;
use self::text::TextParser;
use self::operator::OperatorParser;
use self::symbol::SymbolParser;
use self::line::LineParser;
use self::whitespace::WhitespaceParser;
use self::comment::CommentParser;
use crate::error::KaramelErrorType;

pub struct Parser<'a> {
    tokinizer: Tokinizer<'a>
}

impl<'a> Parser<'a> {
    pub fn new(data: &'a str) -> Parser {
        let mut parser = Parser {
            tokinizer: Tokinizer {
                column: 0,
                line: 0,
                tokens: Vec::new(),
                iter: data.chars().peekable(),
                iter_second: data.chars().peekable(),
                iter_third: data.chars().peekable(),
                data: data.to_string(),
                index: 0
            }
        };

        parser.tokinizer.iter_second.next();
        parser.tokinizer.iter_third.next();
        parser.tokinizer.iter_third.next();
        return parser;
    }

    pub fn tokens(&self) -> Vec<Token> {
        self.tokinizer.tokens.to_vec()
    }

    pub fn parse(&mut self) -> ParseResult {

        let line_parser         = LineParser       {};
        let comment_parser      = CommentParser    {};
        let whitespace_parser   = WhitespaceParser {};
        let number_parser       = NumberParser     {};
        let text_parser_single  = TextParser       { tag:'\'' };
        let text_parser_double  = TextParser       { tag:'"' };
        let operator_parser     = OperatorParser   {};
        let mut symbol_parser   = SymbolParser     {
            keywords: HashMap::new()
        };

        symbol_parser.init_parser();

        while self.tokinizer.is_end() == false {
            let status: Result<(), KaramelErrorType>;

            if line_parser.check(&mut self.tokinizer) {
                status = line_parser.parse(&mut self.tokinizer);
            }
            else if whitespace_parser.check(&mut self.tokinizer) {
                status = whitespace_parser.parse(&mut self.tokinizer);
            }
            else if comment_parser.check(&mut self.tokinizer) {
                status = comment_parser.parse(&mut self.tokinizer);
            }
            else if symbol_parser.check(&mut self.tokinizer) {
                status = symbol_parser.parse(&mut self.tokinizer);
            }
            else if text_parser_single.check(&mut self.tokinizer) {
                status = text_parser_single.parse(&mut self.tokinizer);
            }
            else if text_parser_double.check(&mut self.tokinizer) {
                status = text_parser_double.parse(&mut self.tokinizer);
            }
            else if number_parser.check(&mut self.tokinizer) {
                status = number_parser.parse(&mut self.tokinizer);
            }
            else {
                status = operator_parser.parse(&mut self.tokinizer);
            }

            if status.is_err() {
                return Err(KaramelError {
                    error_type: status.err().unwrap(),
                    line: self.tokinizer.line,
                    column: self.tokinizer.column
                });
            }
        }

        Ok(())
    }
}