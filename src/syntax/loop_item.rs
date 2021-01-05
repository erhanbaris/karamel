use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait, SyntaxFlag};
use crate::compiler::ast::BramaAstType;

pub struct LoopItemParser;

impl SyntaxParserTrait for LoopItemParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        parser.backup();
        parser.cleanup_whitespaces();

        if parser.check_keyword(BramaKeywordType::Break) ||
           parser.check_keyword(BramaKeywordType::Continue) {
            if parser.flags.get().contains(SyntaxFlag::LOOP) {
                let keyword = parser.peek_token().unwrap().token_type.get_keyword();
                parser.consume_token();
                match keyword {
                    BramaKeywordType::Break => return Ok(BramaAstType::Break),
                    BramaKeywordType::Continue => return Ok(BramaAstType::Continue),
                    _ => ()
                };
            }
            else {
                return Err(("break and continue belong to loops", 0, 0));
            }
        }

        parser.restore();
        return Ok(BramaAstType::None);
    }
}