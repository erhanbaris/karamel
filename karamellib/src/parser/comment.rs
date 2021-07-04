use crate::types::*;
use crate::error::KaramelErrorType;

pub struct CommentParser;

impl TokenParser for CommentParser {
    fn check(&self, tokinizer: &mut Tokinizer) -> bool {
        let ch      = tokinizer.get_char();
        let ch_next = tokinizer.get_next_char();
        return (ch == '/' && ch_next == '*') || (ch == '/' && ch_next == '/');
    }

    fn parse(&self, tokinizer: &mut Tokinizer) -> Result<(), KaramelErrorType> {
        let mut ch                   = tokinizer.get_char();
        let mut ch_next              = tokinizer.get_next_char();

        if ch == '/' && ch_next == '*' {
            let mut comment_end = false;

            while !tokinizer.is_end() && !comment_end {
                tokinizer.increase_index();

                if ch.is_new_line() {
                    tokinizer.increate_line();
                }

                ch          = tokinizer.get_char();
                ch_next     = tokinizer.get_next_char();

                if ch == '*' && ch_next == '/' {
                    comment_end = true;
                    tokinizer.increase_index();
                    tokinizer.increase_index();
                }
            }

            if !comment_end {
                return Err(KaramelErrorType::CommentNotFinished);
            }
        }
        else {
            tokinizer.increase_index();
            tokinizer.increase_index();
            ch = tokinizer.get_char();

            while !tokinizer.is_end() &&  ch != '\n' {
                tokinizer.increase_index();

                if ch.is_new_line() {
                    tokinizer.increate_line();
                }

                ch = tokinizer.get_char();
            }
        }

        return Ok(());
    }
}