use std::str;

use std::default::Default;


use std::collections::HashMap;
use crate::types::CharTraits;
use crate::types::*;

pub struct Parser<'a> {
    tokinizer: Tokinizer<'a>,
    keywords: HashMap<&'static str, BramaKeywordType>
}

impl<'a> Parser<'a> {
    pub fn new() -> Parser<'a> {
        let mut parser = Parser {
            keywords: HashMap::new(),
            tokinizer: Default::default()
        };
        parser.init_parser();
        return parser;
    }
    
    fn init_parser(&mut self) {
        for (keyword, keyword_enum) in &KEYWORDS {
            self.keywords.insert(keyword, *keyword_enum);
        }
    }

    fn get_text(&mut self) -> bool {
        let mut ch: char      = '\0';
        let mut ch_next: char;
        let mut symbol        = String::new();

        self.tokinizer.increase_index();

        while !self.tokinizer.is_end() {
            ch      = self.tokinizer.get_char();
            ch_next = self.tokinizer.get_next_char();

            if ch == '\\' && ch_next == '"' {
                symbol.push(ch);
                self.tokinizer.increase_index();
            }
            else if ch == '"' {
                self.tokinizer.increase_index();
                break;
            }
            else {
                symbol.push(ch);
            }

            self.tokinizer.increase_index();
        }

        if ch != '"' {
            return false;
        }

        let token = Token {
            line: self.tokinizer.line,
            column: self.tokinizer.column,
            token_type: BramaTokenType::Text(symbol.to_owned())
        };

        self.tokinizer.add_token(token);
        true
    }

    fn get_symbol(&mut self) -> bool {
        let mut ch: char;
        let mut symbol = String::new();

        while !self.tokinizer.is_end() {
            ch = self.tokinizer.get_char();

            if !ch.is_symbol() && !ch.is_integer() {
                break;
            }

            if ch.is_whitespace() || ch == '\'' || ch == '"' {
                break;
            }
            symbol.push(ch);
            self.tokinizer.increase_index();
        }

        if self.keywords.contains_key(symbol.as_ref() as &str) {
            let keyword = match self.keywords.get(symbol.as_ref() as &str) {
                Some(keyword) => keyword,
                None => &BramaKeywordType::None
            };

            let token = Token {
                line: self.tokinizer.line,
                column: self.tokinizer.column,
                token_type: BramaTokenType::Keyword(*keyword)
            };

            self.tokinizer.add_token(token);
        }
        else {
            let token = Token {
                line: self.tokinizer.line,
                column: self.tokinizer.column,
                token_type: BramaTokenType::Symbol(symbol.to_owned())
            };

            self.tokinizer.add_token(token);
        }
        true
    }

    pub fn parse(&mut self, data: &'static str) {
        self.tokinizer =  Tokinizer {
            column: 0,
            index: 0,
            length: data.chars().count() as i32,
            line: 0,
            tokens: Vec::new(),
            data: data
        };
        let mut ch;
        let mut ch_next;

        while self.tokinizer.is_end() == false {
            ch      = self.tokinizer.get_char() as char;
            ch_next = self.tokinizer.get_next_char();

            if ch.is_new_line() {
                let token = Token {
                    line: self.tokinizer.line,
                    column: self.tokinizer.column,
                    token_type: BramaTokenType::NewLine
                };
                self.tokinizer.add_token(token);
                self.tokinizer.increate_line();
                self.tokinizer.increase_index();
            }
            else if ch.is_whitespace() || (ch == '/' && ch_next == '/'){
                while !self.tokinizer.is_end() &&  ch.is_whitespace() {
                    self.tokinizer.increase_index();

                    if ch.is_new_line() {
                        self.tokinizer.reset_column();
                        self.tokinizer.increate_line();
                    }

                    ch = self.tokinizer.get_char();
                }
                continue;
            }
            else if ch == '/' && ch_next == '*' {
                while !self.tokinizer.is_end() && ch != '*' && ch_next != '/' {
                    self.tokinizer.increase_index();

                    if ch.is_new_line() {
                        self.tokinizer.reset_column();
                        self.tokinizer.increate_line();
                    }

                    ch      = self.tokinizer.get_char();
                    ch_next = self.tokinizer.get_next_char();
                }

                continue;
            }
            else if ch.is_symbol() {
                self.get_symbol();
            }
            else if ch == '"' {
                self.get_text();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_text_1() {
        let mut parser = Parser::new();
        parser.parse("\"erhan barış\"");
        assert_eq!(1, parser.tokinizer.tokens.len());
        for item in parser.tokinizer.tokens.iter() {
            match &item.token_type {
                BramaTokenType::Text(text) => assert_eq!(text, "erhan barış"),
                _ => assert_eq!(true, false)
            }
        }
    }

    #[test]
    fn get_text_2() {
        let mut parser = Parser::new();
        parser.parse("\"erhan barış\"\"\"");
        assert_eq!(2, parser.tokinizer.tokens.len());
        match &parser.tokinizer.tokens[0].token_type {
            BramaTokenType::Text(text) => assert_eq!(text, "erhan barış"),
            _ => assert_eq!(true, false)
        }
        match &parser.tokinizer.tokens[1].token_type {
            BramaTokenType::Text(text) => assert_eq!(text, ""),
            _ => assert_eq!(true, false)
        }
    }

    #[test]
    fn keywords() {
        for (keyword, keyword_enum) in &KEYWORDS {
            let mut parser = Parser::new();
            parser.parse(&keyword);

            assert_eq!(1, parser.tokinizer.tokens.len());
            match &parser.tokinizer.tokens[0].token_type {
                BramaTokenType::Keyword(keyword) => assert_eq!(keyword_enum, keyword),
                _ => assert_eq!(true, false)
            }
        }

        let mut parser = Parser::new();
        parser.parse("_test_");

        assert_eq!(1, parser.tokinizer.tokens.len());
        match &parser.tokinizer.tokens[0].token_type {
            BramaTokenType::Symbol(symbol) => assert_eq!("_test_", symbol),
            _ => assert_eq!(true, false)
        }
    }

    #[test]
    fn new_line() {
        let mut parser = Parser::new();
        parser.parse("\n");

        assert_eq!(1, parser.tokinizer.tokens.len());
        match &parser.tokinizer.tokens[0].token_type {
            BramaTokenType::NewLine => assert_eq!(true, true),
            _ => assert_eq!(true, false)
        }
    }
}