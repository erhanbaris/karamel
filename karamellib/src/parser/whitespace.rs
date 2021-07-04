use crate::types::*;
use crate::error::KaramelErrorType;

pub struct WhitespaceParser;

impl TokenParser for WhitespaceParser {
    fn check(&self, tokinizer: &mut Tokinizer) -> bool {
        let ch = tokinizer.get_char();
        return ch == ' ';
    }

    fn parse(&self, tokinizer: &mut Tokinizer) -> Result<(), KaramelErrorType> {
        let mut whitespace_count: u8 = 0;
        let mut ch                   = tokinizer.get_char();
        let start_column = tokinizer.column;

        while !tokinizer.is_end() && ch == ' ' {
            tokinizer.increase_index();
            whitespace_count += 1;
            ch = tokinizer.get_char();
        }

        tokinizer.add_token(start_column, KaramelTokenType::WhiteSpace(whitespace_count));
        return Ok(());
    }
}