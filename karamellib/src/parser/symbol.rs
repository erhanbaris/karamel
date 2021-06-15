use std::collections::HashMap;
use std::rc::Rc;
use crate::types::*;
use crate::error::BramaErrorType;

pub struct SymbolParser {
    pub keywords: HashMap<&'static str, BramaKeywordType>
}

impl SymbolParser {
    pub fn init_parser(&mut self) {
        for (keyword, keyword_enum) in KEYWORDS.iter() {
            self.keywords.insert(keyword, *keyword_enum);
        }
    }
}

impl TokenParser for SymbolParser {
    fn check(&self, tokinizer: &mut Tokinizer) -> bool {
        let ch = tokinizer.get_char();
        return ch.is_symbol();
    }

    fn parse(&self, tokinizer: &mut Tokinizer) -> Result<(), BramaErrorType> {
        let mut ch: char;
        let start             = tokinizer.index as usize;
        let mut end           = start;
        let start_column = tokinizer.column;

        while !tokinizer.is_end() {
            ch = tokinizer.get_char();

            if !ch.is_symbol() && !ch.is_integer() {
                break;
            }

            if ch.is_whitespace() || ch == '\'' || ch == '"' {
                break;
            }
            end += ch.len_utf8();
            tokinizer.increase_index();
        }
        if self.keywords.contains_key(&tokinizer.data[start..end]) {
            let keyword = match self.keywords.get(&tokinizer.data[start..end]) {
                Some(keyword) => keyword,
                None => &BramaKeywordType::None
            };

            let token_type = match keyword.to_operator() {
                BramaOperatorType::None => BramaTokenType::Keyword(*keyword),
                _                       => BramaTokenType::Operator(keyword.to_operator())
            };
            tokinizer.add_token(start_column as u32, token_type);
            return Ok(());
        }

        tokinizer.add_token(start_column as u32, BramaTokenType::Symbol(Rc::new(tokinizer.data[start..end].to_string())));
        return Ok(());
    }
}