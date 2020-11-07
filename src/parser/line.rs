use crate::types::*;

pub struct LineParser;

impl TokenParser for LineParser {
    fn check(&self, tokinizer: &Tokinizer<'_>) -> bool {
        let ch = tokinizer.get_char();
        return ch.is_new_line();
    }

    fn parse(&self, tokinizer: &mut Tokinizer<'_>) -> Result<BramaTokenType, (String, u32, u32)> {
        tokinizer.increase_index();

        let mut whitespace_count: u8 = 0;
        let mut ch                   = tokinizer.get_char();

        while !tokinizer.is_end() && ch == ' ' {
            tokinizer.increase_index();
            whitespace_count += 1;
            ch = tokinizer.get_char();
        }

        return Ok(BramaTokenType::NewLine(whitespace_count));
    }
}