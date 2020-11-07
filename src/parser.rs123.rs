use std::str;
use std::default::Default;
use std::collections::HashMap;


use crate::types::*;
use crate::parsers::NumberParser;
use crate::parsers::TextParser;

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

    fn get_operator(&mut self) -> BramaStatus {
        let ch       = self.tokinizer.get_char();
        let ch_next  = self.tokinizer.get_next_char();
        let ch_third = self.tokinizer.get_third_char();

        self.tokinizer.increase_index();

        let mut operator_type = match (ch, ch_next, ch_third) {
            ('=', '=', '=') => BramaOperatorType::EqualValue,
            ('!', '=', '=') => BramaOperatorType::NotEqualValue,
            _ =>  BramaOperatorType::None
        };

        if operator_type != BramaOperatorType::None {
            self.tokinizer.increase_index();
            self.tokinizer.increase_index();
        }
        else {
            operator_type = match (ch, ch_next) {
                ('!', '=') => BramaOperatorType::NotEqual,
                ('/', '=') => BramaOperatorType::AssignDivision,
                ('/', '/') => BramaOperatorType::CommentLine,
                ('/', '*') => BramaOperatorType::CommentMultilineStart,
                ('+', '+') => BramaOperatorType::Increment,
                ('+', '=') => BramaOperatorType::AssignAddition,
                ('-', '-') => BramaOperatorType::Deccrement,
                ('-', '=') => BramaOperatorType::AssignSubtraction,
                ('<', '=') => BramaOperatorType::LessEqualThan,
                ('<', '<') => BramaOperatorType::BitwiseLeftShift,
                ('&', '&') => BramaOperatorType::And,
                ('&', '=') => BramaOperatorType::BitwiseAndAssign,
                ('|', '|') => BramaOperatorType::Or,
                ('|', '=') => BramaOperatorType::BitwiseOrAssign,
                ('*', '=') => BramaOperatorType::AssignMultiplication,
                ('*', '/') => BramaOperatorType::CommentMultilineEnd,
                ('=', '=') => BramaOperatorType::Equal,
                ('%', '=') => BramaOperatorType::AssignModulus,
                ('^', '=') => BramaOperatorType::BitwiseXorAssign,
                _ =>  BramaOperatorType::None
            };

            if operator_type != BramaOperatorType::None {
                self.tokinizer.increase_index();
            }
            else {
                operator_type = match ch {
                    '^' => BramaOperatorType::BitwiseXor,
                    '%' => BramaOperatorType::Modulo,
                    '!' => BramaOperatorType::Not,
                    '=' => BramaOperatorType::Assign,
                    '*' => BramaOperatorType::Multiplication,
                    '|' => BramaOperatorType::BitwiseOr,
                    '&' => BramaOperatorType::BitwiseAnd,
                    '<' => BramaOperatorType::LessThan,
                    '-' => BramaOperatorType::Subtraction,
                    '+' => BramaOperatorType::Addition,
                    '/' => BramaOperatorType::Division,
                    '?' => BramaOperatorType::QuestionMark,
                    ':' => BramaOperatorType::ColonMark,
                    '~' => BramaOperatorType::BitwiseNot,
                    '(' => BramaOperatorType::LeftParentheses,
                    ')' => BramaOperatorType::RightParentheses,
                    '[' => BramaOperatorType::SquareBracketStart,
                    ']' => BramaOperatorType::SquareBracketEnd,
                    '{' => BramaOperatorType::CurveBracketStart,
                    '}' => BramaOperatorType::CurveBracketEnd,
                    ',' => BramaOperatorType::Comma,
                    ';' => BramaOperatorType::Semicolon,
                    '.' => BramaOperatorType::Dot,
                    _ => BramaOperatorType::None
                };
            }
        }

        if operator_type == BramaOperatorType::None {
            return BramaStatus::Error(String::from("Char not valid"), self.tokinizer.line, self.tokinizer.column);
        }

        let token = Token {
            line: self.tokinizer.line,
            column: self.tokinizer.column,
            token_type: BramaTokenType::Operator(operator_type)
        };

        self.tokinizer.add_token(token);
        BramaStatus::Ok
    }

    fn get_number(&mut self) -> BramaStatus {
        let mut number_info = NumberParser { };
        let result = number_info.parse(&mut self.tokinizer);

        return match result {
            Err((message, line, column)) => BramaStatus::Error(message, line, column),
            Ok(token_type) => {
                let token = Token {
                    line: self.tokinizer.line,
                    column: self.tokinizer.column,
                    token_type: token_type
                };

                self.tokinizer.add_token(token);
                BramaStatus::Ok
            }
        };
    }

    fn get_text(&mut self, tag: char) -> BramaStatus {
        let mut ch: char      = '\0';
        let mut ch_next: char;
        let mut symbol        = String::new();

        self.tokinizer.increase_index();

        while !self.tokinizer.is_end() {
            ch      = self.tokinizer.get_char();
            ch_next = self.tokinizer.get_next_char();

            if ch == '\\' && ch_next == tag {
                symbol.push(ch);
                self.tokinizer.increase_index();
            }
            else if ch == tag {
                self.tokinizer.increase_index();
                break;
            }
            else {
                symbol.push(ch);
            }

            self.tokinizer.increase_index();
        }

        if ch != tag {
            return BramaStatus::Error(String::from("Missing string deliminator"), self.tokinizer.line, self.tokinizer.column);
        }

        let token = Token {
            line: self.tokinizer.line,
            column: self.tokinizer.column,
            token_type: BramaTokenType::Text(symbol.to_owned())
        };

        self.tokinizer.add_token(token);
        BramaStatus::Ok
    }

    fn get_symbol(&mut self) -> BramaStatus {
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
        return BramaStatus::Ok
    }

    pub fn tokens(&self) -> &Vec<Token> {
        return &self.tokinizer.tokens;
    }

    pub fn parse(&mut self, data: &'static str) {
        self.tokinizer =  Tokinizer {
            column: 0,
            index: 0,
            length: data.chars().count() as u32,
            line: 0,
            tokens: Vec::new(),
            data: data
        };
        let mut ch;
        let mut ch_next;
        let mut status = BramaStatus::Ok;

        while self.tokinizer.is_end() == false {
            status  = BramaStatus::Ok;
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
                status = self.get_symbol();
            }
            else if ch == '"' {
                status = self.get_text('"');
            }
            else if ch == '\'' {
                status = self.get_text('\'');
            }
            else if ch == '.' || (ch >= '0' && ch <= '9') {
                status = self.get_number();
            }
            else {
                status = self.get_operator();
            }
        }

        if status != BramaStatus::Ok {

        }
    }
}
