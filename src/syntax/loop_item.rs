use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait, SyntaxFlag};
use crate::compiler::ast::BramaAstType;

pub struct LoopItemParser;

impl SyntaxParserTrait for LoopItemParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        let index_backup = parser.get_index();
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
                parser.set_index(index_backup);
                return Err(("break and continue belong to loops", 0, 0));
            }
        }

        parser.set_index(index_backup);
        return Ok(BramaAstType::None);
    }
}