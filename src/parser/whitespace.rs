use crate::types::*;

pub struct WhitespaceParser;

impl TokenParser for WhitespaceParser {
    fn check(&self, tokinizer: &Tokinizer) -> bool {
        let ch = tokinizer.get_char();
        return ch == ' ';
    }

    fn parse(&self, tokinizer: &mut Tokinizer) -> Result<BramaTokenType, (String, u32, u32)> {
        let mut whitespace_count: u8 = 0;
        let mut ch                   = tokinizer.get_char();

        while !tokinizer.is_end() && ch == ' ' {
            tokinizer.increase_index();
            whitespace_count += 1;
            ch = tokinizer.get_char();
        }

        return Ok(BramaTokenType::WhiteSpace(whitespace_count));
    }
}