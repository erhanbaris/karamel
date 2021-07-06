use std::rc::Rc;
use crate::types::*;
use crate::error::KaramelErrorType;

pub struct TextParser {
    pub tag: char
}

impl TokenParser for TextParser {
    fn check(&self, tokinizer: &mut Tokinizer) -> bool {
        let ch = tokinizer.get_char();
        return ch == self.tag;
    }

    fn parse(&self, tokinizer: &mut Tokinizer) -> Result<(), KaramelErrorType> {
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
                end += 1; // for tag char
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
            return Err(KaramelErrorType::MissingStringDeliminator);
        }

        tokinizer.add_token(start_column - 1, KaramelTokenType::Text(Rc::new(tokinizer.data[start..end].to_string())));
        return Ok(());
    }
}


#[cfg(test)]
#[test]
fn text_parse_test_1() {
    use crate::types::Tokinizer;

    let data = "\"merhaba dünya\"";
    let mut tokinizer = Tokinizer {
        column: 0,
        line: 0,
        tokens: Vec::new(),
        iter: data.chars().peekable(),
        iter_second: data.chars().peekable(),
        iter_third: data.chars().peekable(),
        data: data.to_string(),
        index: 0
    };

    let parser = TextParser { tag: '"' };
    let parse_result = parser.parse(&mut tokinizer);

    assert_eq!(parse_result.is_ok(), true);
    assert_eq!(tokinizer.tokens.len(), 1);
    assert_eq!(tokinizer.tokens[0].line, 0);
    assert_eq!(tokinizer.tokens[0].start, 0);
    assert_eq!(tokinizer.tokens[0].end, 15);

    match &tokinizer.tokens[0].token_type {
        KaramelTokenType::Text(data) => assert_eq!(&**data, "merhaba dünya"),
        _ => assert_eq!(true, false)
    };
}

#[cfg(test)]
#[test]
fn text_parse_test_2() {
    use crate::types::Tokinizer;

    let data = "'merhaba dünya'";
    let mut tokinizer = Tokinizer {
        column: 0,
        line: 0,
        tokens: Vec::new(),
        iter: data.chars().peekable(),
        iter_second: data.chars().peekable(),
        iter_third: data.chars().peekable(),
        data: data.to_string(),
        index: 0
    };

    let parser = TextParser { tag: '\'' };
    let parse_result = parser.parse(&mut tokinizer);

    assert_eq!(parse_result.is_ok(), true);
    assert_eq!(tokinizer.tokens.len(), 1);
    assert_eq!(tokinizer.tokens[0].line, 0);
    assert_eq!(tokinizer.tokens[0].start, 0);
    assert_eq!(tokinizer.tokens[0].end, 15);

    match &tokinizer.tokens[0].token_type {
        KaramelTokenType::Text(data) => assert_eq!(&**data, "merhaba dünya"),
        _ => assert_eq!(true, false)
    };
}
