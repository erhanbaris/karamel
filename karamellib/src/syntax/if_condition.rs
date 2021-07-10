use std::rc::Rc;

use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::syntax::expression::ExpressionParser;
use crate::compiler::ast::{KaramelAstType, KaramelIfStatementElseItem};
use crate::syntax::block::{SingleLineBlockParser, MultiLineBlockParser};
use crate::error::KaramelErrorType;
use crate::syntax::control::OrParser;

pub struct IfConditiontParser;

impl SyntaxParserTrait for IfConditiontParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        let index_backup = parser.get_index();
        parser.indentation_check()?;

        let indentation = parser.get_indentation();
        let expression = OrParser::parse(parser)?;
        parser.cleanup_whitespaces();

        if parser.match_keyword(KaramelKeywordType::If) {
            parser.cleanup_whitespaces();
            if let None = parser.match_operator(&[KaramelOperatorType::ColonMark]) {
                return Err(KaramelErrorType::ColonMarkMissing);
            }

            parser.cleanup_whitespaces();
            let true_body = match parser.get_newline() {
                (true, _) => {
                    parser.in_indication()?;
                    MultiLineBlockParser::parse(parser)
                },
                (false, _) => SingleLineBlockParser::parse(parser)
            }?;
            parser.set_indentation(indentation);

            if true_body == KaramelAstType::None {
                return Err(KaramelErrorType::IfConditionBodyNotFound);
            }

            parser.cleanup_whitespaces();

            let mut else_body: Option<Rc<KaramelAstType>> = None;
            let mut else_if: Vec<Rc<KaramelIfStatementElseItem>> = Vec::new();

            while parser.is_same_indentation(indentation) {
                if let Some(_) = parser.match_operator(&[KaramelOperatorType::Or]) {
                    parser.cleanup_whitespaces();

                    let else_condition = ExpressionParser::parse(parser)?;

                    if else_body.is_some() {
                        return Err(KaramelErrorType::ElseIsUsed);
                    }

                    match else_condition {
                        KaramelAstType::None => (),
                        _                  => {
                            parser.cleanup_whitespaces();
                            if !parser.match_keyword(KaramelKeywordType::If) {
                                return Err(KaramelErrorType::MissingIf);
                            }
                        }
                    };
                    
                    parser.cleanup_whitespaces();
                    if let None = parser.match_operator(&[KaramelOperatorType::ColonMark]) {
                        return Err(KaramelErrorType::ColonMarkMissing);
                    }

                    else if !else_body.is_none() {
                        return Err(KaramelErrorType::MultipleElseUsageNotValid);
                    }
                    parser.cleanup_whitespaces();
                    
                    let body = match parser.get_newline() {
                        (true, _)  => {
                            parser.in_indication()?;
                            MultiLineBlockParser::parse(parser)
                        },
                        (false, _) => SingleLineBlockParser::parse(parser)
                    }?;

                    if body == KaramelAstType::None {
                        return Err(KaramelErrorType::IfConditionBodyNotFound);
                    }

                    parser.set_indentation(indentation);

                    match else_condition {
                        KaramelAstType::None => else_body = Some(Rc::new(body)),
                        _                  => else_if.push(Rc::new(KaramelIfStatementElseItem::new(Rc::new(else_condition), Rc::new(body))))
                    };
                }
                else {
                    break;
                }

                if let Err(_) = parser.indentation_check() {
                    break;
                }
            }

            let assignment_ast = KaramelAstType::IfStatement {
                condition: Rc::new(expression),
                body: Rc::new(true_body),
                else_body,
                else_if: else_if.to_vec()
            };

            parser.set_indentation(indentation);
            return Ok(assignment_ast);
        }
        
        parser.set_index(index_backup);
        return Ok(KaramelAstType::None);
    }
}
