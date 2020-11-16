use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use crate::types::*;
use crate::syntax::util::*;
use crate::syntax::SyntaxParser;
use crate::syntax::control::ExpressionParser;

pub struct PrimativeParser;

impl PrimativeParser {
    fn parse_basic_primatives(parser: &SyntaxParser) -> AstResult {
        let token = parser.peek_token();
        if token.is_err() {
            return Err(("Syntax error", 0, 0));
        }

        let result = match &token.unwrap().token_type {
            BramaTokenType::Integer(int)      => Ok(BramaAstType::Primative(BramaPrimative::Number(*int as f64))),
            BramaTokenType::Double(double)    => Ok(BramaAstType::Primative(BramaPrimative::Number(*double))),
            BramaTokenType::Text(text)        => Ok(BramaAstType::Primative(BramaPrimative::Text(text))),
            BramaTokenType::Keyword(keyword)  => {
                match keyword {
                    BramaKeywordType::True  => Ok(BramaAstType::Primative(BramaPrimative::Bool(true))),
                    BramaKeywordType::False => Ok(BramaAstType::Primative(BramaPrimative::Bool(false))),
                    BramaKeywordType::Empty => Ok(BramaAstType::Primative(BramaPrimative::Empty)),
                    _ => Ok(BramaAstType::None)
                }
            },
            BramaTokenType::Operator(BramaOperatorType::ColonMark) => {
                let next_token = parser.next_token();
                if next_token.is_err() {
                    return Ok(BramaAstType::None)
                }

                match &next_token.unwrap().token_type {
                    BramaTokenType::Symbol(symbol) => {
                        parser.consume_token();

                        let mut hasher = DefaultHasher::new();
                        symbol.hash(&mut hasher);

                        Ok(BramaAstType::Primative(BramaPrimative::Atom(hasher.finish())))
                    },
                    _ => Ok(BramaAstType::None)
                }
            },
            _ => Ok(BramaAstType::None)
        };

        match result {
            Ok(BramaAstType::None) => Err(("Syntax error", token.unwrap().line, token.unwrap().column)),
            Ok(ast) => {
                parser.consume_token();
                Ok(ast)
            },
            Err((message, line, column)) => Err((message, line, column))
        }
    }

    fn parse_list(parser: &SyntaxParser) -> AstResult {
        if parser.match_operator(&[BramaOperatorType::SquareBracketStart]).is_some() {
            let mut ast_vec   = Vec::new();
            parser.clear_whitespaces();

            loop {
                if parser.check_operator(&BramaOperatorType::SquareBracketEnd) {
                    break;
                }

                parser.clear_whitespaces();

                let ast = ExpressionParser::parse(parser);
                if is_ast_empty(&ast) {
                    return err_or_message(&ast, "Invalid list item");
                }
                
                ast_vec.push(Box::new(ast.unwrap()));

                parser.clear_whitespaces();
                if parser.match_operator(&[BramaOperatorType::Comma]).is_none()  {
                    break;
                }
            }

            if parser.match_operator(&[BramaOperatorType::SquareBracketEnd]).is_none() {
                return Err(("Array not closed", 0, 0));
            }

            return Ok(BramaAstType::Primative(BramaPrimative::List(ast_vec)));
        }

        return Ok(BramaAstType::None);
    }

    fn parse_symbol(parser: &SyntaxParser) -> AstResult {
        parser.clear_whitespaces();
        let token = parser.peek_token();
        if token.is_err() {
            return Err(("Syntax error", 0, 0));
        }

        if let BramaTokenType::Symbol(symbol) = &token.unwrap().token_type {
            parser.consume_token();
            return Ok(BramaAstType::Symbol(symbol));
        }
        return Ok(BramaAstType::None);
    }

    fn parse_parenthesis(parser: &SyntaxParser) -> AstResult {
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

        return Ok(BramaAstType::None);
    }
}

impl SyntaxParserTrait for PrimativeParser {
    type Item = PrimativeParser;

    fn parse(parser: &SyntaxParser) -> AstResult {
        return map_parser(parser, &[Self::parse_list, Self::parse_parenthesis, Self::parse_symbol, Self::parse_basic_primatives]);
    }
}