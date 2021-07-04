use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::compiler::ast::KaramelAstType;

pub struct NewlineParser;

impl SyntaxParserTrait for NewlineParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        let mut result = KaramelAstType::None;
        loop {
            if let Ok(token) = parser.peek_token() {
                match token.token_type {
                    KaramelTokenType::NewLine(_) => {
                        parser.indentation_check()?;
                        result = KaramelAstType::NewLine;
                        parser.consume_token();
                        continue;
                    },
                    KaramelTokenType::WhiteSpace(_) => {
                        result = KaramelAstType::NewLine;
                        parser.consume_token();
                        continue;
                    },
                    _ => {
                        result = KaramelAstType::None;
                        break;
                    }
                }
            }
            break;
        }

        Ok(result)
    }
}