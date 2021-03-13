use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::compiler::ast::BramaAstType;

pub struct NewlineParser;

impl SyntaxParserTrait for NewlineParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        let mut result = BramaAstType::None;
        loop {
            if let Ok(token) = parser.peek_token() {
                match token.token_type {
                    BramaTokenType::NewLine(_) => {
                        parser.indentation_check()?;
                        result = BramaAstType::NewLine;
                        parser.consume_token();
                        continue;
                    },
                    BramaTokenType::WhiteSpace(_) => {
                        result = BramaAstType::NewLine;
                        parser.consume_token();
                        continue;
                    },
                    _ => {
                        result = BramaAstType::None;
                        break;
                    }
                }
            }
            break;
        }

        Ok(result)
    }
}