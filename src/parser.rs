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

    fn get_number(&mut self) -> BramaStatus {
        let mut index               = 0;
        let mut is_minus            = false;
        let mut dot_place           = 0;
        let mut before_comma :u64   = 0;
        let mut after_comma  :u64   = 0;
        let mut is_double           = false;
        let mut number_system       = BramaNumberSystem::Decimal;
        let mut e_used              = false;
        let mut e_after             = 0;
        let mut plus_used           = false;
        let mut ch                  = self.tokinizer.get_char();
        let ch_next                 = self.tokinizer.get_next_char();

        if ch == '0' {
            match ch_next {
                'b' | 'B' => {
                    number_system = BramaNumberSystem::Binary;
                    self.tokinizer.increase_index();        
                },
                'x' | 'X' => {
                    number_system = BramaNumberSystem::Hexadecimal;
                    self.tokinizer.increase_index();        
                },
                '0'..='7' => number_system = BramaNumberSystem::Octal,
                _ => {}
            };
        }

        while !self.tokinizer.is_end() {
            if ch == '-' {
                if (is_minus || (before_comma > 0 || after_comma > 0)) && !e_used {
                    break;
                }

                is_minus = true;
            }

            else if ch == '+' {
                if (plus_used || (before_comma > 0 || after_comma > 0)) && !e_used {
                    break;
                }
    
                plus_used = true;
            }

            else if index != 0 && (ch == 'e' || ch == 'E') {
                e_used = true;
            }

            else if ch == '.' {
                if is_double {
                    return BramaStatus::MultipleDotOnDouble(self.tokinizer.line, self.tokinizer.column);
                }
    
                is_double = true;
            }

            else if !e_used && number_system == BramaNumberSystem::Decimal && ch.is_ascii_digit() {
                if is_double {
                    dot_place += 1;
    
                    after_comma *= u64::pow(10, 1);
                    after_comma += ch as u64 - '0' as u64;
                    
                }
                else {
                    before_comma *= u64::pow(10, 1);
                    before_comma += ch as u64 - '0' as u64;
                }
            }

            else if !e_used && number_system == BramaNumberSystem::Binary && (ch == '0' || ch == '1') {
                before_comma = before_comma << 1;

                let tmp_ch = ch.to_digit(2);
                if tmp_ch.is_some() {
                    before_comma += tmp_ch.unwrap() as u64;
                }
            }

            else if !e_used && number_system == BramaNumberSystem::Hexadecimal && ch.is_ascii_hexdigit() {    
                before_comma = before_comma << 4;

                let tmp_ch = ch.to_digit(16);
                if tmp_ch.is_some() {
                    before_comma += tmp_ch.unwrap() as u64;
                }
            }

            else if !e_used && number_system == BramaNumberSystem::Octal && ch >= '0' && ch <= '7' {
                before_comma = before_comma << 3;

                let tmp_ch = ch.to_digit(8);
                if tmp_ch.is_some() {
                    before_comma += tmp_ch.unwrap() as u64;
                }
            }

            else if e_used && ch >= '0' && ch <= '9' {
                e_after *= u32::pow(10, 1);
                
                let tmp_ch = ch.to_digit(10);
                if tmp_ch.is_some() {
                    e_after += tmp_ch.unwrap();
                }
            }
            else {
                break;
            }

            self.tokinizer.increase_index();
            ch      = self.tokinizer.get_char();
            index += 1;
        }

        let mut token_type = match is_double {
            false => BramaTokenType::Integer(before_comma as i64),
            true => {
                let num = before_comma as f64 + (after_comma as f64 * f64::powi(10.0, -1 * dot_place as i32));
                BramaTokenType::Double(num as f64)
            }
        };

        if e_used {
            if is_minus {
                token_type = match token_type {
                    BramaTokenType::Integer(num) => BramaTokenType::Integer(num / i64::pow(10, e_after)),
                    BramaTokenType::Double(num) => BramaTokenType::Double(num / f64::powi(10.0, e_after as i32)),
                    _ => BramaTokenType::None
                };
            } else {
                token_type = match token_type {
                    BramaTokenType::Integer(num) => BramaTokenType::Integer(num * i64::pow(10, e_after)),
                    BramaTokenType::Double(num) => BramaTokenType::Double(num * f64::powi(10.0, e_after as i32)),
                    _ => BramaTokenType::None
                };
            }
        }

        let token = Token {
            line: self.tokinizer.line,
            column: self.tokinizer.column,
            token_type: token_type
        };

        self.tokinizer.add_token(token);
        BramaStatus::Ok
        /*
    t_token_ptr token = (t_token_ptr)BRAMA_MALLOC(sizeof (t_token));
    if (NULL == token) {
        context->status = out_of_memory_error(context);
        return 0;
    }

    if (!isDouble) {
        token->type = TOKEN_INTEGER;
        token->double_ = beforeTheComma;
    } else {
        token->type    = TOKEN_DOUBLE;
        token->double_ = (beforeTheComma + (afterTheComma * pow(10, -1 * dotPlace)));
    }

    if (e_used) {
        if (isMinus) {
            token->double_ = token->double_ / (double)pow((double)10, (double)e_after);
        } else {
            token->double_ = token->double_ * (double)pow((double)10, (double)e_after);
        }
    }

    token->current = start;
    token->line    = tokinizer->line;

    if (isMinus && !e_used)
        token->double_ *= -1;

    vec_push(tokinizer->tokens, token);
    return BRAMA_OK;
    */
    }

    fn get_text(&mut self) -> BramaStatus {
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
            return BramaStatus::MissingStringDemininator(self.tokinizer.line, self.tokinizer.column);
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
                status = self.get_text();
            }
            else if ch == '.' || (ch >= '0' && ch <= '9') {
                status = self.get_number();
            }
        }

        if status != BramaStatus::Ok {

        }
    }
}

#[warn(unused_macros)]
macro_rules! test_number {
    ($name:ident, $type:ident, $text:expr, $result:expr) => {
        // The macro will expand into the contents of this block.
        #[test]
        fn $name () {
            let mut parser = Parser::new();
            parser.parse($text);
    
            assert_eq!(1, parser.tokinizer.tokens.len());
            match &parser.tokinizer.tokens[0].token_type {
                BramaTokenType::$type(num) => assert_eq!(*num, $result),
                _ => assert_eq!(true, false)
            }
        }
    };
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

    test_number!(integer_1, Integer, "1024", 1024);
    test_number!(integer_2, Integer, " 1024000 ", 1024000);
    test_number!(integer_3, Integer, "123", 123);
    test_number!(integer_4, Integer, "9223372036854775807", 9223372036854775807);
    test_number!(integer_5, Integer, "0999999", 999999);
    
    test_number!(hex_1, Integer, "0x12", 18);
    test_number!(hex_2, Integer, "0xffffff", 16777215);
    test_number!(hex_3, Integer, "0x1FFFFFFFFFFFFF", 9007199254740991);

    test_number!(oct_1, Integer, "062", 50);
    test_number!(oct_2, Integer, "06211111111111", 430723863113);

    test_number!(binary_1, Integer, "0b10000000000000000000000000000000", 2147483648);
    test_number!(binary_2, Integer, "0b01111111100000000000000000000000", 2139095040);
    test_number!(binary_3, Integer, "0b01", 1);
    test_number!(binary_4, Integer, "0B00000000011111111111111111111111", 8388607);


    test_number!(double_1, Double, "1024.0", 1024.0);
    #[test]
    fn double_2() {
        let mut parser = Parser::new();
        parser.parse(" .1024000 ");

        assert_eq!(1, parser.tokinizer.tokens.len());
        match &parser.tokinizer.tokens[0].token_type {
            BramaTokenType::Double(num) => assert_eq!(0.1024 - *num < 1e-10, true),
            _ => assert_eq!(true, false)
        }
    }
    test_number!(double_3, Double, "099999.9", 99999.9);
    
}