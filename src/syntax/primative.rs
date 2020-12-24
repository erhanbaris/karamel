use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::rc::Rc;

use crate::types::*;
use crate::syntax::util::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::syntax::control::ExpressionParser;
use crate::compiler::value::BramaPrimative;
use crate::compiler::ast::{BramaAstType, BramaDictItem};

pub struct PrimativeParser;

impl PrimativeParser {
    pub fn parse_basic_primatives(parser: &SyntaxParser) -> AstResult {
        parser.backup();
        parser.cleanup_whitespaces();

        let token = parser.peek_token();
        if token.is_err() {
            return Ok(BramaAstType::None);
        }

        let result = match &token.unwrap().token_type {
            BramaTokenType::Integer(int)      => Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Number(*int as f64)))),
            BramaTokenType::Double(double)    => Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Number(*double)))),
            BramaTokenType::Text(text)        => Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Text(Rc::clone(text))))),
            BramaTokenType::Keyword(keyword)  => {
                match keyword {
                    BramaKeywordType::True  => Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Bool(true)))),
                    BramaKeywordType::False => Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Bool(false)))),
                    BramaKeywordType::Empty => Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Empty))),
                    _ => Ok(BramaAstType::None)
                }
            },
            BramaTokenType::Operator(BramaOperatorType::ColonMark) => {
                let next_token = parser.next_token();
                if next_token.is_err() {
                    parser.restore();
                    return Ok(BramaAstType::None)
                }

                match &next_token.unwrap().token_type {
                    BramaTokenType::Symbol(symbol) => {
                        parser.consume_token();

                        let mut hasher = DefaultHasher::new();
                        symbol.hash(&mut hasher);

                        Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Atom(hasher.finish()))))
                    },
                    _ => Ok(BramaAstType::None)
                }
            },
            _ => Ok(BramaAstType::None)
        };

        match result {
            Ok(BramaAstType::None) => {
                parser.restore();
                Ok(BramaAstType::None)
            },
            Ok(ast) => {
                parser.consume_token();
                Ok(ast)
            },
            Err((message, line, column)) => Err((message, line, column))
        }
    }

    pub fn parse_list(parser: &SyntaxParser) -> AstResult {
        parser.backup();
        if parser.match_operator(&[BramaOperatorType::SquareBracketStart]).is_some() {
            let mut ast_vec   = Vec::new();
            parser.cleanup_whitespaces();

            loop {
                if parser.check_operator(&BramaOperatorType::SquareBracketEnd) {
                    break;
                }

                parser.cleanup_whitespaces();

                let ast = ExpressionParser::parse(parser);
                if is_ast_empty(&ast) {
                    return err_or_message(&ast, "Invalid list item");
                }
                
                ast_vec.push(Box::new(ast.unwrap()));

                parser.cleanup_whitespaces();
                if parser.match_operator(&[BramaOperatorType::Comma]).is_none()  {
                    break;
                }
            }

            if parser.match_operator(&[BramaOperatorType::SquareBracketEnd]).is_none() {
                return Err(("Array not closed", 0, 0));
            }

            return Ok(BramaAstType::List(ast_vec));
        }

        parser.restore();
        return Ok(BramaAstType::None);
    }

    pub fn parse_dict(parser: &SyntaxParser) -> AstResult {
        parser.backup();
        if parser.match_operator(&[BramaOperatorType::CurveBracketStart]).is_some() {
            let mut dict_items   = Vec::new();
            parser.cleanup();

            loop {
                if parser.check_operator(&BramaOperatorType::CurveBracketEnd) {
                    break;
                }

                parser.cleanup();

                let key_ast = Self::parse_basic_primatives(parser);
                if is_ast_empty(&key_ast) {
                    return err_or_message(&key_ast, "Dictionary key not valid");
                }
                
                /* Check dictionary key */
                let key = match key_ast {
                    Ok(BramaAstType::Primative(primative)) => {
                        match &*primative {
                            BramaPrimative::Text(_) => primative.clone(),
                            _ =>  return Err((": Dictionary key not valid", 0, 0))
                        }
                    },
                    _ => return Err((": Dictionary key not valid", 0, 0))
                };

                parser.cleanup();

                if parser.match_operator(&[BramaOperatorType::ColonMark]).is_none()  {
                    return Err((": required", 0, 0));
                }

                parser.cleanup();
                let value = ExpressionParser::parse(parser);
                if is_ast_empty(&value) {
                    return err_or_message(&value, "Dictionary value not valid");
                }
  
                dict_items.push(Box::new(BramaDictItem {
                    key: key,
                    value: Rc::new(value.unwrap())
                }));

                parser.cleanup();
                if parser.match_operator(&[BramaOperatorType::Comma]).is_none()  {
                    break;
                }
            }

            if parser.match_operator(&[BramaOperatorType::CurveBracketEnd]).is_none() {
                return Err(("Dict not closed", 0, 0));
            }

            return Ok(BramaAstType::Dict(dict_items));
        }

        parser.restore();
        return Ok(BramaAstType::None);
    }

    pub fn parse_symbol(parser: &SyntaxParser) -> AstResult {
        parser.backup();
        parser.cleanup_whitespaces();
        let token = parser.peek_token();
        if token.is_err() {
            return Ok(BramaAstType::None);
        }

        if let BramaTokenType::Symbol(symbol) = &token.unwrap().token_type {
            parser.consume_token();
            return Ok(BramaAstType::Symbol(symbol.to_string()));
        }
        parser.restore();
        return Ok(BramaAstType::None);
    }

    pub fn parse_parenthesis(parser: &SyntaxParser) -> AstResult {
        parser.backup();
        if parser.match_operator(&[BramaOperatorType::LeftParentheses]).is_some() {
            
            let ast = ExpressionParser::parse(parser);
            if is_ast_empty(&ast) {
                return err_or_message(&ast, "Invalid expression");
            }

            if parser.match_operator(&[BramaOperatorType::RightParentheses]).is_none() {
                return Err(("Parentheses not closed", 0, 0));
            }

            return Ok(ast.unwrap());
        }

        parser.restore();
        return Ok(BramaAstType::None);
    }
}

impl SyntaxParserTrait for PrimativeParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        return map_parser(parser, &[Self::parse_dict, Self::parse_list, Self::parse_parenthesis, Self::parse_symbol, Self::parse_basic_primatives]);
    }
}