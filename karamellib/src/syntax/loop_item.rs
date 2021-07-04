use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait, SyntaxFlag};
use crate::compiler::ast::KaramelAstType;
use crate::error::KaramelErrorType;

pub struct LoopItemParser;

impl SyntaxParserTrait for LoopItemParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        let index_backup = parser.get_index();
        parser.cleanup_whitespaces();

        if parser.check_keyword(KaramelKeywordType::Break) ||
           parser.check_keyword(KaramelKeywordType::Continue) {
            if parser.flags.get().contains(SyntaxFlag::LOOP) {
                let keyword = parser.peek_token().unwrap().token_type.get_keyword();
                parser.consume_token();
                match keyword {
                    KaramelKeywordType::Break => return Ok(KaramelAstType::Break),
                    KaramelKeywordType::Continue => return Ok(KaramelAstType::Continue),
                    _ => ()
                };
            }
            else {
                parser.set_index(index_backup);
                return Err(KaramelErrorType::BreakAndContinueBelongToLoops);
            }
        }

        parser.set_index(index_backup);
        return Ok(KaramelAstType::None);
    }
}