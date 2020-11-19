use std::collections::HashMap;
use crate::types::*;

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

    fn parse(&self, tokinizer: &mut Tokinizer) -> Result<BramaTokenType, (&'static str, u32, u32)> {
        let mut ch: char;
        let start             = tokinizer.index as usize;
        let mut end           = start;

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

            return match keyword.to_operator() {
                BramaOperatorType::None => Ok(BramaTokenType::Keyword(*keyword)),
                _                       => Ok(BramaTokenType::Operator(keyword.to_operator()))
            }
        }

        return Ok(BramaTokenType::Symbol(tokinizer.data[start..end].to_string()));
    }
}