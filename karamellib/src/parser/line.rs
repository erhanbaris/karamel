use crate::types::*;
use crate::error::KaramelErrorType;

pub struct LineParser;

impl TokenParser for LineParser {
    fn check(&self, tokinizer: &mut Tokinizer) -> bool {
        let ch = tokinizer.get_char();
        return ch.is_new_line();
    }

    fn parse(&self, tokinizer: &mut Tokinizer) -> Result<(), KaramelErrorType> {
        tokinizer.increase_index();

        let mut whitespace_count: u32 = 0;
        let start_column = tokinizer.column;
        let mut ch                   = tokinizer.get_char();

        while !tokinizer.is_end() && ch == ' ' {
            tokinizer.increase_index();
            whitespace_count += 1;
            ch = tokinizer.get_char();
        }

        tokinizer.increate_line();
        tokinizer.add_token(start_column, KaramelTokenType::NewLine(whitespace_count as u8));
        tokinizer.column = whitespace_count;

        return Ok(());
    }
}