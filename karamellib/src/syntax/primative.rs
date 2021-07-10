use std::rc::Rc;

use crate::types::*;
use crate::syntax::util::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::syntax::expression::ExpressionParser;
use crate::compiler::value::KaramelPrimative;
use crate::compiler::ast::{KaramelAstType, KaramelDictItem};
use crate::error::KaramelErrorType;

pub struct PrimativeParser;

impl PrimativeParser {
    pub fn parse_basic_primatives(parser: &SyntaxParser) -> AstResult {
        let index_backup = parser.get_index();
        parser.cleanup_whitespaces();

        let token = parser.peek_token();
        if token.is_err() {
            return Ok(KaramelAstType::None);
        }

        let result = match &token.unwrap().token_type {
            KaramelTokenType::Integer(int)      => Ok(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(*int as f64)))),
            KaramelTokenType::Double(double)    => Ok(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(*double)))),
            KaramelTokenType::Text(text)        => Ok(KaramelAstType::Primative(Rc::new(KaramelPrimative::Text(Rc::clone(text))))),
            KaramelTokenType::Keyword(keyword)  => {
                match keyword {
                    KaramelKeywordType::True  => Ok(KaramelAstType::Primative(Rc::new(KaramelPrimative::Bool(true)))),
                    KaramelKeywordType::False => Ok(KaramelAstType::Primative(Rc::new(KaramelPrimative::Bool(false)))),
                    KaramelKeywordType::Empty => Ok(KaramelAstType::Primative(Rc::new(KaramelPrimative::Empty))),
                    _ => Ok(KaramelAstType::None)
                }
            },
            _ => Ok(KaramelAstType::None)
        };

        match result {
            Ok(KaramelAstType::None) => {
                parser.set_index(index_backup);
                Ok(KaramelAstType::None)
            },
            Ok(ast) => {
                parser.consume_token();
                Ok(ast)
            },
            _ => result
        }
    }

    pub fn parse_list(parser: &SyntaxParser) -> AstResult {
        let index_backup = parser.get_index();
        if parser.match_operator(&[KaramelOperatorType::SquareBracketStart]).is_some() {
            let mut ast_vec   = Vec::new();
            parser.cleanup_whitespaces();

            loop {
                if parser.check_operator(&KaramelOperatorType::SquareBracketEnd) {
                    break;
                }

                parser.cleanup_whitespaces();

                let ast = ExpressionParser::parse(parser);
                if is_ast_empty(&ast) {
                    return err_or_message(ast, KaramelErrorType::InvalidListItem);
                }
                
                ast_vec.push(Rc::new(ast.unwrap()));

                parser.cleanup_whitespaces();
                if parser.match_operator(&[KaramelOperatorType::Comma]).is_none()  {
                    break;
                }
            }

            if parser.match_operator(&[KaramelOperatorType::SquareBracketEnd]).is_none() {
                return Err(KaramelErrorType::ArrayNotClosed);
            }

            return Ok(KaramelAstType::List(ast_vec));
        }

        parser.set_index(index_backup);
        return Ok(KaramelAstType::None);
    }

    pub fn parse_dict(parser: &SyntaxParser) -> AstResult {
        let index_backup = parser.get_index();
        if parser.match_operator(&[KaramelOperatorType::CurveBracketStart]).is_some() {
            let mut dict_items   = Vec::new();
            parser.cleanup();

            loop {
                if parser.check_operator(&KaramelOperatorType::CurveBracketEnd) {
                    break;
                }

                parser.cleanup();

                let key_ast = Self::parse_basic_primatives(parser);
                if is_ast_empty(&key_ast) {
                    return err_or_message(key_ast, KaramelErrorType::DictionaryKeyNotValid);
                }
                
                /* Check dictionary key */
                let key = match key_ast {
                    Ok(KaramelAstType::Primative(primative)) => {
                        match &*primative {
                            KaramelPrimative::Text(_) => primative.clone(),
                            _ =>  {
                                return Err(KaramelErrorType::DictionaryKeyNotValid);
                            }
                        }
                    },
                    _ => return Err(KaramelErrorType::DictionaryKeyNotValid)
                };

                parser.cleanup();

                if parser.match_operator(&[KaramelOperatorType::ColonMark]).is_none()  {
                    return Err(KaramelErrorType::ColonMarkMissing);
                }

                parser.cleanup();
                let value = ExpressionParser::parse(parser);
                if is_ast_empty(&value) {
                    return err_or_message(value, KaramelErrorType::DictionaryValueNotValid);
                }
  
                dict_items.push(Rc::new(KaramelDictItem {
                    key,
                    value: Rc::new(value.unwrap())
                }));

                parser.cleanup();
                if parser.match_operator(&[KaramelOperatorType::Comma]).is_none()  {
                    break;
                }
            }

            if parser.match_operator(&[KaramelOperatorType::CurveBracketEnd]).is_none() {
                return Err(KaramelErrorType::DictNotClosed);
            }

            return Ok(KaramelAstType::Dict(dict_items));
        }

        parser.set_index(index_backup);
        return Ok(KaramelAstType::None);
    }

    pub fn parse_symbol(parser: &SyntaxParser) -> AstResult {
        let index_backup = parser.get_index();
        parser.cleanup_whitespaces();
        let token = parser.peek_token();
        if token.is_err() {
            return Ok(KaramelAstType::None);
        }

        if let KaramelTokenType::Symbol(symbol) = &token.unwrap().token_type {
            parser.consume_token();
            return Ok(KaramelAstType::Symbol(symbol.to_string()));
        }
        parser.set_index(index_backup);
        return Ok(KaramelAstType::None);
    }

    pub fn parse_module_path(parser: &SyntaxParser) -> AstResult {
        let index_backup = parser.get_index();
        parser.cleanup_whitespaces();
        let token = parser.peek_token();
        if token.is_err() {
            return Ok(KaramelAstType::None);
        }

        if let KaramelTokenType::Symbol(symbol) = &token.unwrap().token_type {
            let mut symbol_definitions: Vec<String> = Vec::new();
            symbol_definitions.push(symbol.to_string());

            parser.consume_token();
            loop {
                if let Some(_) = parser.match_operator(&[KaramelOperatorType::ColonMark]) {
                    if let Some(_) = parser.match_operator(&[KaramelOperatorType::ColonMark]) {
                        if let KaramelTokenType::Symbol(inner_symbol) = &parser.peek_token().unwrap().token_type {
                            parser.consume_token();
                            symbol_definitions.push(inner_symbol.to_string());
                            continue;
                        }
                        else {
                            parser.set_index(index_backup);
                            return Ok(KaramelAstType::None);
                        }
                    }
                    else {
                        parser.set_index(index_backup);
                        return Ok(KaramelAstType::None);
                    }
                }
                break;
            }
            
            if symbol_definitions.len() > 1 {
                return Ok(KaramelAstType::ModulePath(symbol_definitions.to_vec()));
            }
        }

        parser.set_index(index_backup);
        return Ok(KaramelAstType::None);
    }

    pub fn parse_parenthesis(parser: &SyntaxParser) -> AstResult {
        let index_backup = parser.get_index();
        if parser.match_operator(&[KaramelOperatorType::LeftParentheses]).is_some() {
            
            let ast = ExpressionParser::parse(parser);
            if is_ast_empty(&ast) {
                return err_or_message(ast, KaramelErrorType::InvalidExpression);
            }

            if parser.match_operator(&[KaramelOperatorType::RightParentheses]).is_none() {
                return Err(KaramelErrorType::ParenthesesNotClosed);
            }

            return Ok(ast.unwrap());
        }

        parser.set_index(index_backup);
        return Ok(KaramelAstType::None);
    }
}

impl SyntaxParserTrait for PrimativeParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        return map_parser(parser, &[Self::parse_dict, Self::parse_list, Self::parse_parenthesis, Self::parse_module_path, Self::parse_symbol, Self::parse_basic_primatives]);
    }
}