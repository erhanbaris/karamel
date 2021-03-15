use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::syntax::expression::ExpressionParser;
use crate::compiler::ast::{BramaAstType, BramaIfStatementElseItem};
use crate::syntax::block::{SingleLineBlockParser, MultiLineBlockParser};

pub struct IfConditiontParser;

impl SyntaxParserTrait for IfConditiontParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        let index_backup = parser.get_index();
        parser.indentation_check()?;

        if parser.match_keyword(BramaKeywordType::If) {
            let indentation = parser.get_indentation();
            parser.cleanup_whitespaces();

            let expression = ExpressionParser::parse(parser);
            match expression {
                Ok(BramaAstType::None) => return expression,
                Ok(_) => (),
                Err(_) => return expression
            };

            parser.cleanup_whitespaces();
            if let None = parser.match_operator(&[BramaOperatorType::ColonMark]) {
                parser.set_index(index_backup);
                return Err(BramaError::ColonMarkMissing);
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
                parser.set_index(index_backup);
                return Err(BramaError::IfConditionBodyNotFound);
            }

            parser.cleanup_whitespaces();

            let mut else_body: Option<Box<BramaAstType>> = None;
            let mut else_if: Vec<Box<BramaIfStatementElseItem>> = Vec::new();

            while parser.is_same_indentation(indentation) {
                if parser.match_keyword(BramaKeywordType::Else) {
                    parser.cleanup_whitespaces();

                    let else_condition = ExpressionParser::parse(parser)?;

                    if else_body.is_some() {
                        parser.set_index(index_backup);
                        return Err(BramaError::ElseIsUsed);
                    }

                    if let None = parser.match_operator(&[BramaOperatorType::ColonMark]) {
                        parser.set_index(index_backup);
                        return Err(BramaError::ColonMarkMissing);
                    }
                    else if !else_body.is_none() {
                        parser.set_index(index_backup);
                        return Err(BramaError::MultipleElseUsageNotValid);
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
                        parser.set_index(index_backup);
                        return Err(BramaError::IfConditionBodyNotFound);
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
                condition: Box::new(expression.unwrap()),
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
