use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::compiler::ast::BramaAstType;

pub struct NewlineParser;

impl SyntaxParserTrait for NewlineParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        let mut result = BramaAstType::None;
        loop {
            if let Ok(token) = parser.peek_token() {
                if let BramaTokenType::NewLine(_) = token.token_type {
                    parser.indentation_check()?;
                    result = BramaAstType::NewLine;
                    continue;
                }
            }
            break;
        }

        Ok(result)
    }
}