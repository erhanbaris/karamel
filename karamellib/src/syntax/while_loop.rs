use std::rc::Rc;

use crate::syntax::util::map_parser;
use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait, SyntaxFlag};
use crate::compiler::ast::{KaramelAstType};
use crate::syntax::block::{SingleLineBlockParser, MultiLineBlockParser};
use crate::syntax::expression::ExpressionParser;
use crate::error::KaramelErrorType;

use super::assignment::AssignmentParser;
#[derive(Debug)]
pub enum LoopType {
    Simple(Rc<KaramelAstType>),
    Scalar { 
        variable: Rc<KaramelAstType>, 
        expression: Rc<KaramelAstType> 
    },
    Endless
}

pub struct WhileLoopParser;

impl SyntaxParserTrait for WhileLoopParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        let index_backup = parser.get_index();
        parser.indentation_check()?;

        let indentation = parser.get_indentation();

        let loop_type = match parser.match_keywords(&[KaramelKeywordType::Endless, KaramelKeywordType::While]) {
            Some(KaramelKeywordType::Endless) => LoopType::Endless, // Endless loop

            Some(KaramelKeywordType::While) => {
                let control = map_parser(parser, &[AssignmentParser::parse, ExpressionParser::parse])?;
                let loop_type = match &control {
                    KaramelAstType::None => return Ok(control),
                    KaramelAstType::Assignment { variable, operator, expression } => {
                        match operator { 
                            KaramelOperatorType::Assign => (),
                            _ => return Err(KaramelErrorType::AssignOperatorRequiredForLoop)
                        };
                        LoopType::Scalar {
                            variable: variable.clone(),
                            expression: expression.clone()
                        }
                    },
                    _ => LoopType::Simple(Rc::new(control.clone()))
                };

                loop_type
            },
            _ => return Ok(KaramelAstType::None)
        };

        println!("{:?}", loop_type);
        
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
        döngü doğru 
        döngü sayaç = 1, sayaç < 10, ++sayaç
        */
        else if parser.match_keyword(KaramelKeywordType::While) {
            parser.cleanup_whitespaces();
            let indentation = parser.get_indentation();

            let control = map_parser(parser, &[AssignmentParser::parse, ExpressionParser::parse])?;
            let loop_type = match &control {
                KaramelAstType::None => return Ok(control),
                KaramelAstType::Assignment { variable, operator, expression } => {
                    match operator { 
                        KaramelOperatorType::Assign => (),
                        _ => return Err(KaramelErrorType::AssignOperatorRequiredForLoop)
                    };
                    LoopType::Scalar {
                        variable: variable.clone(),
                        expression: expression.clone()
                    }
                },
                _ => LoopType::Simple(Rc::new(control.clone()))
            };

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
            return Ok(KaramelAstType::Loop {
                control: Box::new(control),
                body: Box::new(body)
            });
        }
        
        parser.set_index(index_backup);
        return Ok(KaramelAstType::None);
    }
}
