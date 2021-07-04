use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::syntax::util::map_parser;
use crate::syntax::primative::PrimativeParser;
use crate::syntax::func_call::FuncCallParser;
use crate::syntax::util::is_ast_empty;
use crate::compiler::ast::KaramelAstType;
use crate::compiler::value::KaramelPrimative;
use crate::syntax::expression::ExpressionParser;
use crate::error::KaramelErrorType;


use std::rc::Rc;

pub struct UnaryParser;

impl SyntaxParserTrait for UnaryParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        let ast = map_parser(parser, &[Self::parse_prefix_unary, Self::parse_suffix_unary, FuncCallParser::parse, PrimativeParser::parse])?;
        
        let index_backup = parser.get_index();
        parser.cleanup_whitespaces();
        
        if parser.match_operator(&[KaramelOperatorType::SquareBracketStart]).is_some() {
            parser.cleanup_whitespaces();

            let indexer_ast = ExpressionParser::parse(parser)?;
            parser.cleanup_whitespaces();

            if parser.match_operator(&[KaramelOperatorType::SquareBracketEnd]).is_some() {
                return Ok(KaramelAstType::Indexer { body: Box::new(ast), indexer: Box::new(indexer_ast) });   
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
                        KaramelOperatorType::Increment,
                        KaramelOperatorType::Deccrement]) {
                        return Ok(KaramelAstType::SuffixUnary(operator, Box::new(KaramelAstType::Symbol(token.token_type.get_symbol().to_string()))));
                    }
                }
            },
            _ => ()
        };
        
        parser.set_index(index_backup);
        return Ok(KaramelAstType::None);
    }

    pub fn parse_indexer(ast: Box<KaramelAstType>, parser: &SyntaxParser) -> AstResult {
        let index_backup = parser.get_index();
        if parser.match_operator(&[KaramelOperatorType::SquareBracketStart]).is_some() {
            parser.cleanup_whitespaces();

            let indexer_ast = ExpressionParser::parse(parser);
            parser.cleanup_whitespaces();

            if parser.match_operator(&[KaramelOperatorType::SquareBracketEnd]).is_some() && !is_ast_empty(&indexer_ast) {
                return Ok(KaramelAstType::Indexer { body: ast, indexer: Box::new(indexer_ast.unwrap()) });   
            }
        }

        parser.set_index(index_backup);
        return Ok(KaramelAstType::None);
    }

    fn parse_prefix_unary(parser: &SyntaxParser) -> AstResult {
        let index_backup = parser.get_index();

        if let Some(operator) = parser.match_operator(&[KaramelOperatorType::Addition,
            KaramelOperatorType::Subtraction,
            KaramelOperatorType::Increment,
            KaramelOperatorType::Deccrement,
            KaramelOperatorType::Not]) {
            parser.cleanup_whitespaces();

            let mut unary_ast = KaramelAstType::None;
            let token         = &parser.peek_token().unwrap();

            match operator {
                /* +1024 -1024 */
                KaramelOperatorType::Addition | KaramelOperatorType::Subtraction => {
                    let opt = match operator {
                        KaramelOperatorType::Addition    => 1 as f64,
                        KaramelOperatorType::Subtraction => -1 as f64,
                        _ => 1 as f64
                    };

                    parser.consume_token();
                    match token.token_type {
                        KaramelTokenType::Integer(integer) => return Ok(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(integer as f64 * opt)))),
                        KaramelTokenType::Double(double) => return Ok(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(double * opt)))),
                        _ => {
                            parser.set_index(index_backup);
                            return Err(KaramelErrorType::UnaryWorksWithNumber);
                        }
                    }
                },

                /* ++variable, --variable*/
                KaramelOperatorType::Increment | KaramelOperatorType::Deccrement => {
                    if token.token_type.is_symbol() {
                        unary_ast = KaramelAstType::Symbol(token.token_type.get_symbol().to_string());
                        parser.consume_token();
                    }
                },

                KaramelOperatorType::Not => {
                    let expression = UnaryParser::parse(parser);
                    unary_ast = match expression {
                        Ok(KaramelAstType::None) => {
                            parser.set_index(index_backup);
                            return Err(KaramelErrorType::InvalidUnaryOperation);
                        },
                        Ok(ast) => ast,
                        Err(_) => {
                            parser.set_index(index_backup);
                            return Err(KaramelErrorType::InvalidUnaryOperation);
                        }
                    };
                }
                _ => { 
                    parser.set_index(index_backup);
                    return Err(KaramelErrorType::InvalidUnaryOperation);
                }
            }

            return match unary_ast {
                KaramelAstType::None => {
                    parser.set_index(index_backup);
                    Err(KaramelErrorType::InvalidUnaryOperation)
                },
                _ => Ok(KaramelAstType::PrefixUnary(operator, Box::new(unary_ast)))
            };
        }

        return Ok(KaramelAstType::None);
    }
}