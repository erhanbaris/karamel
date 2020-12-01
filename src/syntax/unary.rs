use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::syntax::util::map_parser;
use crate::syntax::primative::PrimativeParser;
use crate::syntax::func_call::FuncCallParser;
use crate::compiler::ast::BramaAstType;
use crate::compiler::value::BramaPrimative;

use std::rc::Rc;

pub struct UnaryParser;

impl SyntaxParserTrait for UnaryParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        return map_parser(parser, &[Self::parse_prefix_unary, Self::parse_suffix_unary, FuncCallParser::parse, PrimativeParser::parse]);
    }
}

impl UnaryParser {
    fn parse_suffix_unary(parser: &SyntaxParser) -> AstResult {
        parser.backup();
        match &parser.peek_token() {
            Ok(token) => {
                if token.token_type.is_symbol() {
                    parser.consume_token();
                    parser.clear_whitespaces();

                    if let Some(operator) = parser.match_operator(&[
                        BramaOperatorType::Increment,
                        BramaOperatorType::Deccrement]) {
                        return Ok(BramaAstType::SuffixUnary(operator, Box::new(BramaAstType::Symbol(token.token_type.get_symbol().to_string()))));
                    }
                    ()
                }
            },
            _ => ()
        };
        
        parser.restore();
        return Ok(BramaAstType::None);
    }

    fn parse_prefix_unary(parser: &SyntaxParser) -> AstResult {
        if let Some(operator) = parser.match_operator(&[BramaOperatorType::Addition,
            BramaOperatorType::Subtraction,
            BramaOperatorType::Increment,
            BramaOperatorType::Deccrement,
            BramaOperatorType::Not]) {
            parser.clear_whitespaces();

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
                        _ => return Err(("Unary works with number", 0, 0))
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
                        Ok(BramaAstType::None) => return Err(("Invalid unary expression", 0, 0)),
                        Ok(ast) => ast,
                        Err(_) => return Err(("Invalid unary expression", 0, 0))
                    };
                }
                _ => return Err(("Invalid unary operation", 0, 0))
            }

            return match unary_ast {
                BramaAstType::None => Err(("Invalid unary operation", 0, 0)),
                _ => Ok(BramaAstType::PrefixUnary(operator, Box::new(unary_ast)))
            };
        }

        return Ok(BramaAstType::None);
    }
}