use std::collections::HashMap;
use crate::types::*;

pub struct SymbolParser {
    pub keywords: HashMap<&'static str, BramaKeywordType>
}

impl SymbolParser {
    pub fn init_parser(&mut self) {
        for (keyword, keyword_enum) in &KEYWORDS {
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
        let mut symbol = String::new();

        while !tokinizer.is_end() {
            ch = tokinizer.get_char();

            if !ch.is_symbol() && !ch.is_integer() {
                break;
            }

            if ch.is_whitespace() || ch == '\'' || ch == '"' {
                break;
            }
            symbol.push(ch);
            tokinizer.increase_index();
        }

        if self.keywords.contains_key(symbol.as_ref() as &str) {
            let keyword = match self.keywords.get(symbol.as_ref() as &str) {
                Some(keyword) => keyword,
                None => &BramaKeywordType::None
            };

            return Ok(BramaTokenType::Keyword(*keyword));
        }
        return Ok(BramaTokenType::Symbol(symbol.to_owned()));
    }
}