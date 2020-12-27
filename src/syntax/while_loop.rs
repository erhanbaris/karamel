use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait, SyntaxFlag};
use crate::compiler::ast::{BramaAstType};
use crate::syntax::block::{SingleLineBlockParser, MultiLineBlockParser};

pub struct WhileLoopParser;

impl SyntaxParserTrait for WhileLoopParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        parser.backup();
        parser.indentation_check()?;

        if parser.match_keyword(BramaKeywordType::Endless) {
            let indentation = parser.get_indentation();

            parser.cleanup_whitespaces();
            if let None = parser.match_operator(&[BramaOperatorType::ColonMark]) {
                return Err(("':' missing", 0, 0));
            }

            parser.cleanup_whitespaces();
            let parser_flags  = parser.flags.get();
            parser.flags.set(parser_flags | SyntaxFlag::LOOP);

            let body = match parser.get_newline() {
                (true, _) => {
                    parser.in_indication()?;
                    MultiLineBlockParser::parse(parser)
                },
                (false, _) => SingleLineBlockParser::parse(parser)
            }?;

            parser.set_indentation(indentation);
            parser.flags.set(parser_flags);

            parser.set_indentation(indentation);
            return Ok(BramaAstType::EndlessLoop(Box::new(body)));
        }
        
        parser.restore();
        return Ok(BramaAstType::None);
    }
}
