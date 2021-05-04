use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::syntax::expression::ExpressionParser;
use crate::compiler::ast::{BramaAstType, BramaIfStatementElseItem};
use crate::syntax::block::{SingleLineBlockParser, MultiLineBlockParser};
use crate::error::BramaErrorType;
use crate::syntax::control::OrParser;

pub struct IfConditiontParser;

impl SyntaxParserTrait for IfConditiontParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        let index_backup = parser.get_index();
        parser.indentation_check()?;

        let indentation = parser.get_indentation();
        let expression = OrParser::parse(parser)?;
        parser.cleanup_whitespaces();

        if parser.match_keyword(BramaKeywordType::If) {
            parser.cleanup_whitespaces();
            if let None = parser.match_operator(&[BramaOperatorType::ColonMark]) {
                return Err(BramaErrorType::ColonMarkMissing);
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

            if true_body == BramaAstType::None {
                return Err(BramaErrorType::IfConditionBodyNotFound);
            }

            parser.cleanup_whitespaces();

            let mut else_body: Option<Box<BramaAstType>> = None;
            let mut else_if: Vec<Box<BramaIfStatementElseItem>> = Vec::new();

            while parser.is_same_indentation(indentation) {
                if let Some(_) = parser.match_operator(&[BramaOperatorType::Or]) {
                    parser.cleanup_whitespaces();

                    let else_condition = ExpressionParser::parse(parser)?;

                    if else_body.is_some() {
                        return Err(BramaErrorType::ElseIsUsed);
                    }

                    match else_condition {
                        BramaAstType::None => (),
                        _                  => {
                            parser.cleanup_whitespaces();
                            if !parser.match_keyword(BramaKeywordType::If) {
                                return Err(BramaErrorType::MissingIf);
                            }
                        }
                    };
                    
                    parser.cleanup_whitespaces();
                    if let None = parser.match_operator(&[BramaOperatorType::ColonMark]) {
                        return Err(BramaErrorType::ColonMarkMissing);
                    }

                    else if !else_body.is_none() {
                        return Err(BramaErrorType::MultipleElseUsageNotValid);
                    }
                    parser.cleanup_whitespaces();
                    
                    let body = match parser.get_newline() {
                        (true, _)  => {
                            parser.in_indication()?;
                            MultiLineBlockParser::parse(parser)
                        },
                        (false, _) => SingleLineBlockParser::parse(parser)
                    }?;

                    if body == BramaAstType::None {
                        return Err(BramaErrorType::IfConditionBodyNotFound);
                    }

                    parser.set_indentation(indentation);

                    match else_condition {
                        BramaAstType::None => else_body = Some(Box::new(body)),
                        _                  => else_if.push(Box::new(BramaIfStatementElseItem::new(Box::new(else_condition), Box::new(body))))
                    };
                }
                else {
                    break;
                }

                if let Err(_) = parser.indentation_check() {
                    break;
                }
            }

            let assignment_ast = BramaAstType::IfStatement {
                condition: Box::new(expression),
                body: Box::new(true_body),
                else_body,
                else_if: else_if.to_vec()
            };

            parser.set_indentation(indentation);
            return Ok(assignment_ast);
        }
        
        parser.set_index(index_backup);
        return Ok(BramaAstType::None);
    }
}
