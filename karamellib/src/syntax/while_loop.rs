use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait, SyntaxFlag};
use crate::compiler::ast::{BramaAstType};
use crate::syntax::block::{SingleLineBlockParser, MultiLineBlockParser};
use crate::syntax::expression::ExpressionParser;

pub struct WhileLoopParser;

impl SyntaxParserTrait for WhileLoopParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        let index_backup = parser.get_index();
        parser.indentation_check()?;

        /*
        sonsuz
        endless
         */
        if parser.match_keyword(BramaKeywordType::Endless) {
            let indentation = parser.get_indentation();

            parser.cleanup_whitespaces();
            if let None = parser.match_operator(&[BramaOperatorType::ColonMark]) {
                parser.set_index(index_backup);
                return Err(BramaError::ColonMarkMissing);
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

        /* 
        döngü doğru ise
        do true while 
        */
        else if parser.match_keyword(BramaKeywordType::WhileStartPart) {
            parser.cleanup_whitespaces();
            let indentation = parser.get_indentation();

            let control = ExpressionParser::parse(parser)?;
            match control {
                BramaAstType::None => return Ok(control),
                _ => ()
            };

            parser.cleanup_whitespaces();
            if !parser.match_keyword(BramaKeywordType::WhileEndPart) {
                parser.set_indentation(indentation);
                return Err(BramaError::WhileStatementNotValid);
            }

            parser.cleanup_whitespaces();
            if let None = parser.match_operator(&[BramaOperatorType::ColonMark]) {
                parser.set_indentation(indentation);
                return Err(BramaError::ColonMarkMissing);
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
            return Ok(BramaAstType::WhileLoop {
                control: Box::new(control),
                body: Box::new(body)
            });
        }
        
        parser.set_index(index_backup);
        return Ok(BramaAstType::None);
    }
}
