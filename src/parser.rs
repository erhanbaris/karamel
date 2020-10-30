use std::str;

use std::collections::HashMap;
use crate::types::CharTraits;
use crate::types::*;

pub struct Parser {
    keywords: HashMap<&'static str, BramaKeywordType>
}

impl Parser {
    pub fn new() -> Parser {
        let mut parser = Parser {
            keywords: HashMap::new()
        };
        parser.init_parser();
        return parser;
    }
    
    fn init_parser(&mut self) {
        for (keyword, operator) in &KEYWORDS {
            self.keywords.insert(keyword, *operator);
        }
    }

    fn get_symbol(&mut self, tokinizer: &mut Tokinizer) {
        let mut ch: char;
        let mut chars: Vec<char> = Vec::new();
        while !tokinizer.is_end() {
            ch = tokinizer.get_char();

            if !ch.is_symbol() && !ch.is_integer() {
                break;
            }

            if ch.is_whitespace() || ch == '\'' || ch == '"' {
                break;
            }

            chars.push(ch);
            tokinizer.increase_index();
        }

        let symbol: String = chars.iter().collect();
        let symbol = &symbol[..];
        if self.keywords.contains_key(&symbol) {
            let keyword = match self.keywords.get(&symbol) {
                Some(keyword) => keyword,
                None => &BramaKeywordType::None
            };

            let token = KeywordToken {
                line: tokinizer.line,
                column: tokinizer.column,
                keyword: *keyword
            };

            tokinizer.add_token(Box::new(token));
        }
        
        /*

        char_ptr data       = NULL;
        string_stream_get(stream, &data);
        string_stream_destroy(stream);
        BRAMA_FREE(stream);

        int_ptr keywordInfo = (int_ptr)map_get(&tokinizer->keywords, data);

        if (keywordInfo) {
            t_token_ptr token = (t_token_ptr)BRAMA_MALLOC(sizeof (t_token));
            if (NULL == token) {
                context->status = out_of_memory_error(context);
                return 0;
            }

            token->type       = TOKEN_KEYWORD;
            token->current    = tokinizer->column;
            token->line       = tokinizer->line;
            token->keyword    = *keywordInfo;

            vec_push(tokinizer->tokens, token);
            BRAMA_FREE(data);
        } else {
            t_token_ptr token  = (t_token_ptr)BRAMA_MALLOC(sizeof (t_token));
            if (NULL == token) {
                context->status = out_of_memory_error(context);
                return 0;
            }
            token->type        = TOKEN_SYMBOL;
            token->current     = tokinizer->column;
            token->line        = tokinizer->line;
            token->char_ptr    = data;

            vec_push(tokinizer->tokens, token);
        }

        return BRAMA_OK;*/
    }

    pub fn parse(&mut self, data: &'static str) {
        let mut tokinizer =  Tokinizer {
            column: 0,
            index: 0,
            length: data.len() as i32,
            line: 0,
            tokens: Vec::new(),
            data: data
        };
        let mut ch;
        let mut ch_next;

        while tokinizer.is_end() == false {
            ch      = tokinizer.get_char() as char;
            ch_next = tokinizer.get_next_char();

            if ch.is_new_line() {
                let token = OperatorToken {
                    line: tokinizer.line,
                    column: tokinizer.column,
                    operator: BramaOperatorType::NewLine
                };
                tokinizer.add_token(Box::new(token));
                tokinizer.increate_line();
                tokinizer.increase_index();
            }
            else if ch.is_whitespace() ||
                (ch == '/' && ch_next == '/'){
                while !tokinizer.is_end() &&  ch.is_whitespace() {
                    tokinizer.increase_index();

                    if ch.is_new_line() {
                        tokinizer.reset_column();
                        tokinizer.increate_line();
                    }

                    ch = tokinizer.get_char();
                }
                continue;
            }
            else if ch == '/' && ch_next == '*' {
                while !tokinizer.is_end() && ch != '*' && ch_next != '/' {
                    tokinizer.increase_index();

                    if ch.is_new_line() {
                        tokinizer.reset_column();
                        tokinizer.increate_line();
                    }

                    ch      = tokinizer.get_char();
                    ch_next = tokinizer.get_next_char();
                }

                continue;
            }
            else if ch.is_symbol() {
                self.get_symbol(&mut tokinizer);
                /*let state = getSymbol(context, tokinizer);
                 RESULT_CHECK(state);
                 continue;*/
            }
        }
    }
}