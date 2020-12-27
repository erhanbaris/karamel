use std::rc::Rc;
use crate::types::*;

pub struct TextParser {
    pub tag: char
}

impl TokenParser for TextParser {
    fn check(&self, tokinizer: &mut Tokinizer) -> bool {
        let ch = tokinizer.get_char();
        return ch == self.tag;
    }

    fn parse(&self, tokinizer: &mut Tokinizer) -> Result<(), (&'static str, u32, u32)> {
        tokinizer.increase_index();

        let mut ch: char      = '\0';
        let mut ch_next: char;
        let start             = tokinizer.index as usize;
        let start_column = tokinizer.column;
        let mut end           = start;

        while !tokinizer.is_end() {
            ch      = tokinizer.get_char();
            ch_next = tokinizer.get_next_char();

            if ch == '\\' && ch_next == self.tag {
                end += ch.len_utf8();
                tokinizer.increase_index();
            }
            else if ch == self.tag {
                tokinizer.increase_index();
                break;
            }
            else {
                end += ch.len_utf8();
            }

            tokinizer.increase_index();
        }

        if ch != self.tag {
            return Err(("Missing string deliminator", tokinizer.line, tokinizer.column));
        }

        tokinizer.add_token(start_column, BramaTokenType::Text(Rc::new(tokinizer.data[start..end].to_string())));
        return Ok(());
    }
}