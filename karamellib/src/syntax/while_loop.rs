use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait, SyntaxFlag};
use crate::compiler::ast::{KaramelAstType};
use crate::syntax::block::{SingleLineBlockParser, MultiLineBlockParser};
use crate::syntax::expression::ExpressionParser;
use crate::error::KaramelErrorType;

pub struct WhileLoopParser;

impl SyntaxParserTrait for WhileLoopParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        let index_backup = parser.get_index();
        parser.indentation_check()?;

        /*
        sonsuz
        endless
         */
        if parser.match_keyword(KaramelKeywordType::Endless) {
            let indentation = parser.get_indentation();

            parser.cleanup_whitespaces();
            if let None = parser.match_operator(&[KaramelOperatorType::ColonMark]) {
                return Err(KaramelErrorType::ColonMarkMissing);
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
            return Ok(KaramelAstType::EndlessLoop(Box::new(body)));
        }

        /* 
        döngü doğru ise
        do true while 
        */
        else if parser.match_keyword(KaramelKeywordType::WhileStartPart) {
            parser.cleanup_whitespaces();
            let indentation = parser.get_indentation();

            let control = ExpressionParser::parse(parser)?;
            match control {
                KaramelAstType::None => return Ok(control),
                _ => ()
            };

            parser.cleanup_whitespaces();
            if !parser.match_keyword(KaramelKeywordType::WhileEndPart) {
                parser.set_indentation(indentation);
                return Err(KaramelErrorType::WhileStatementNotValid);
            }

            parser.cleanup_whitespaces();
            if let None = parser.match_operator(&[KaramelOperatorType::ColonMark]) {
                parser.set_indentation(indentation);
                return Err(KaramelErrorType::ColonMarkMissing);
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
            return Ok(KaramelAstType::WhileLoop {
                control: Box::new(control),
                body: Box::new(body)
            });
        }
        
        parser.set_index(index_backup);
        return Ok(KaramelAstType::None);
    }
}
