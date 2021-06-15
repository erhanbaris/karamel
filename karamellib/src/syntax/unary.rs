use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::syntax::util::map_parser;
use crate::syntax::primative::PrimativeParser;
use crate::syntax::func_call::FuncCallParser;
use crate::syntax::util::is_ast_empty;
use crate::compiler::ast::BramaAstType;
use crate::compiler::value::BramaPrimative;
use crate::syntax::expression::ExpressionParser;
use crate::error::BramaErrorType;


use std::rc::Rc;

pub struct UnaryParser;

impl SyntaxParserTrait for UnaryParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        let ast = map_parser(parser, &[Self::parse_prefix_unary, Self::parse_suffix_unary, FuncCallParser::parse, PrimativeParser::parse])?;
        
        let index_backup = parser.get_index();
        parser.cleanup_whitespaces();
        
        if parser.match_operator(&[BramaOperatorType::SquareBracketStart]).is_some() {
            parser.cleanup_whitespaces();

            let indexer_ast = ExpressionParser::parse(parser)?;
            parser.cleanup_whitespaces();

            if parser.match_operator(&[BramaOperatorType::SquareBracketEnd]).is_some() {
                return Ok(BramaAstType::Indexer { body: Box::new(ast), indexer: Box::new(indexer_ast) });   
            }
        }

        parser.set_index(index_backup);
        return Ok(ast);
    }
}

impl UnaryParser {
    fn parse_suffix_unary(parser: &SyntaxParser) -> AstResult {
        let index_backup = parser.get_index();
        match &parser.peek_token() {
            Ok(token) => {
                if token.token_type.is_symbol() {
                    parser.consume_token();
                    parser.cleanup_whitespaces();

                    if let Some(operator) = parser.match_operator(&[
                        BramaOperatorType::Increment,
                        BramaOperatorType::Deccrement]) {
                        return Ok(BramaAstType::SuffixUnary(operator, Box::new(BramaAstType::Symbol(token.token_type.get_symbol().to_string()))));
                    }
                }
            },
            _ => ()
        };
        
        parser.set_index(index_backup);
        return Ok(BramaAstType::None);
    }

    pub fn parse_indexer(ast: Box<BramaAstType>, parser: &SyntaxParser) -> AstResult {
        let index_backup = parser.get_index();
        if parser.match_operator(&[BramaOperatorType::SquareBracketStart]).is_some() {
            parser.cleanup_whitespaces();

            let indexer_ast = ExpressionParser::parse(parser);
            parser.cleanup_whitespaces();

            if parser.match_operator(&[BramaOperatorType::SquareBracketEnd]).is_some() && !is_ast_empty(&indexer_ast) {
                return Ok(BramaAstType::Indexer { body: ast, indexer: Box::new(indexer_ast.unwrap()) });   
            }
        }

        parser.set_index(index_backup);
        return Ok(BramaAstType::None);
    }

    fn parse_prefix_unary(parser: &SyntaxParser) -> AstResult {
        let index_backup = parser.get_index();

        if let Some(operator) = parser.match_operator(&[BramaOperatorType::Addition,
            BramaOperatorType::Subtraction,
            BramaOperatorType::Increment,
            BramaOperatorType::Deccrement,
            BramaOperatorType::Not]) {
            parser.cleanup_whitespaces();

            let mut unary_ast = BramaAstType::None;
            let token         = &parser.peek_token().unwrap();

            match operator {
                /* +1024 -1024 */
                BramaOperatorType::Addition | BramaOperatorType::Subtraction => {
                    let opt = match operator {
                        BramaOperatorType::Addition    => 1 as f64,
                        BramaOperatorType::Subtraction => -1 as f64,
                        _ => 1 as f64
                    };

                    parser.consume_token();
                    match token.token_type {
                        BramaTokenType::Integer(integer) => return Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Number(integer as f64 * opt)))),
                        BramaTokenType::Double(double) => return Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Number(double * opt)))),
                        _ => {
                            parser.set_index(index_backup);
                            return Err(BramaErrorType::UnaryWorksWithNumber);
                        }
                    }
                },

                /* ++variable, --variable*/
                BramaOperatorType::Increment | BramaOperatorType::Deccrement => {
                    if token.token_type.is_symbol() {
                        unary_ast = BramaAstType::Symbol(token.token_type.get_symbol().to_string());
                        parser.consume_token();
                    }
                },

                BramaOperatorType::Not => {
                    let expression = UnaryParser::parse(parser);
                    unary_ast = match expression {
                        Ok(BramaAstType::None) => {
                            parser.set_index(index_backup);
                            return Err(BramaErrorType::InvalidUnaryOperation);
                        },
                        Ok(ast) => ast,
                        Err(_) => {
                            parser.set_index(index_backup);
                            return Err(BramaErrorType::InvalidUnaryOperation);
                        }
                    };
                }
                _ => { 
                    parser.set_index(index_backup);
                    return Err(BramaErrorType::InvalidUnaryOperation);
                }
            }

            return match unary_ast {
                BramaAstType::None => {
                    parser.set_index(index_backup);
                    Err(BramaErrorType::InvalidUnaryOperation)
                },
                _ => Ok(BramaAstType::PrefixUnary(operator, Box::new(unary_ast)))
            };
        }

        return Ok(BramaAstType::None);
    }
}