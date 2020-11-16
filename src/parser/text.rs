use crate::types::*;

pub struct TextParser {
    pub tag: char
}

impl TokenParser for TextParser {
    fn check(&self, tokinizer: &mut Tokinizer) -> bool {
        let ch = tokinizer.get_char();
        return ch == self.tag;
    }

    fn parse(&self, tokinizer: &mut Tokinizer) -> Result<BramaTokenType, (&'static str, u32, u32)> {
        let mut ch: char      = '\0';
        let mut ch_next: char;
        let start             = tokinizer.index as usize;
        let mut end           = start;

        tokinizer.increase_index();

        while !tokinizer.is_end() {
            ch      = tokinizer.get_char();
            ch_next = tokinizer.get_next_char();

            if ch == '\\' && ch_next == self.tag {
                end += 1;
                tokinizer.increase_index();
            }
            else if ch == self.tag {
                tokinizer.increase_index();
                break;
            }
            else {
                end += 1;
            }

            tokinizer.increase_index();
        }

        if ch != self.tag {
            return Err(("Missing string deliminator", tokinizer.line, tokinizer.column));
        }

        let end = tokinizer.data.char_indices().map(|(i, _)| i).nth(end).unwrap();
        return Ok(BramaTokenType::Text(&tokinizer.data[start..end]));
    }
}