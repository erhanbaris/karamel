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
        let mut index             = 0;
        let mut isMinus             = false;
        let mut dotPlace             = 0;
        let mut beforeTheComma    = 0.0;
        let mut afterTheComma     = 0.0;
        let mut start             = tokinizer->column;
        let mut isDouble            = false;
        brama_number_system type = NUMBER_SYSTEM_DECIMAL;
        let mut e_used              = false;
        let mut e_after              = 0;
        let mut plus_used           = false;
        let mut ch                  = self.tokinizer.get_char();
        let mut chNext              = self.tokinizer.get_next_char();
        let mut ch                  = self.tokinizer.get_char();
        let mut chNext              = self.tokinizer.get_next_char();

        while !self.tokinizer.is_end() {
            if (ch == '-') {
                if ((is_minus || (beforeTheComma > 0 || afterTheComma > 0)) && !e_used)
                break;

                isMinus = true;
            }
        }
        /*
        size_t index             = 0;
    bool isMinus             = false;
    int dotPlace             = 0;
    double beforeTheComma    = 0;
    double afterTheComma     = 0;
    size_t start             = tokinizer->column;
    bool isDouble            = false;
    char ch                  = getChar(tokinizer);
    char chNext              = getNextChar(tokinizer);
    brama_number_system type = NUMBER_SYSTEM_DECIMAL;
    bool e_used              = false;
    int e_after              = 0;
    bool plus_used           = false;

    while (!isEnd(tokinizer)) {
        if (ch == '-') {
            if ((isMinus || (beforeTheComma > 0 || afterTheComma > 0)) && !e_used)
                break;

            isMinus = true;
        }

        else if (ch == '+') {
            if ((plus_used || (beforeTheComma > 0 || afterTheComma > 0)) && !e_used)
                break;

            plus_used = true;
        }

        else if (index == 0 && ch == '0' && chNext == 'x') { // HEX
            type = NUMBER_SYSTEM_HEXADECIMAL;
            increase(tokinizer);
        }

        else if (index != 0 && (ch == 'e' || ch == 'E')) {
            e_used = true;
        }

        else if (index == 0 && ch == '0' && (chNext >= '0' && chNext <= '9')) { // OCT
            type = NUMBER_SYSTEM_OCTAL;
        }

        else if (ch == '.') {
            /*if (chNext == '.')
                break;*/

            if (isDouble) {
                return BRAMA_MULTIPLE_DOT_ON_DOUBLE;
            }

            isDouble = true;
        }

        else if (!e_used && type == NUMBER_SYSTEM_DECIMAL && (ch >= '0' && ch <= '9')) {
            if (isDouble) {
                ++dotPlace;

                afterTheComma *= (int)pow(10, 1);
                afterTheComma += ch - '0';
            }
            else {
                beforeTheComma *= (int)pow(10, 1);
                beforeTheComma += ch - '0';
            }
        }

        else if (!e_used && type == NUMBER_SYSTEM_HEXADECIMAL && ((ch >= '0' && ch <= '9') || (ch >= 'a' && ch <= 'f') || (ch >= 'A' && ch <= 'F'))) {
            ch = (ch <= '9') ? ch - '0' : (ch & 0x7) + 9;

            beforeTheComma = (uint64_t)beforeTheComma << 4;
            beforeTheComma += (int)ch;
        }

        else if (!e_used && type == NUMBER_SYSTEM_OCTAL && ((ch >= '0' && ch <= '7'))) {
            int num = ch - '0';
            int dec_value = 0;

            int base = 1;
            int temp = num;
            while (temp) {
                int last_digit = temp % 10;
                temp = temp / 10;
                dec_value += last_digit * base;
                base = base * 8;
            }

            beforeTheComma = (uint64_t)beforeTheComma << 3;
            beforeTheComma += (int)dec_value;
        }

        else if (e_used && (ch >= '0' && ch <= '9')) {
            e_after *= (int)pow(10, 1);
            e_after += ch - '0';
        }
        else
            break;

        increase(tokinizer);
        ch     = getChar(tokinizer);
        chNext = getNextChar(tokinizer);
        ++index;
    }

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
        }

        if status != BramaStatus::Ok {

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