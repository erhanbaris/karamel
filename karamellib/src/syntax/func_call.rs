use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait, SyntaxFlag, ExtensionSyntaxParser};
use crate::syntax::expression::ExpressionParser;
use crate::syntax::primative::PrimativeParser;
use crate::compiler::ast::BramaAstType;
use crate::syntax::util::map_parser;

pub struct FuncCallParser;

impl SyntaxParserTrait for FuncCallParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        if parser.flags.get().contains(SyntaxFlag::IN_DICT_INDEXER) {
            return Ok(BramaAstType::None);
        }

        let index = parser.get_index();
        parser.cleanup_whitespaces();
        let token = parser.peek_token();

        if token.is_ok() {
            let function_name = map_parser(parser, &[PrimativeParser::parse_function_map, PrimativeParser::parse_symbol])?;

            match function_name {
                BramaAstType::None => (),
                _ => { 
                    let parse_result = FuncCallParser::parse_suffix(Box::new(function_name), parser)?;
                    match parse_result {
                        BramaAstType::None => (),
                        _ => return Ok(parse_result)
                    }; 
                }
            };
        }
        
        parser.set_index(index);
        return Ok(BramaAstType::None);
    }
}

impl ExtensionSyntaxParser for FuncCallParser {
    fn parsable(parser: &SyntaxParser) -> bool {
        if parser.flags.get().contains(SyntaxFlag::IN_DICT_INDEXER) {
            return false;
        }

        parser.check_operator(&BramaOperatorType::LeftParentheses)
    }

    fn parse_suffix(ast: Box<BramaAstType>, parser: &SyntaxParser) -> AstResult {

        let index_backup = parser.get_index();
        let parser_flags  = parser.flags.get();
        parser.cleanup_whitespaces();

        if let Some(_) = parser.match_operator(&[BramaOperatorType::LeftParentheses]) {
            let mut arguments = Vec::new();

            let inner_parser_flags  = parser.flags.get();
            parser.flags.set(parser_flags | SyntaxFlag::IN_FUNCTION_ARG);

            /* Parse function call arguments */
            let mut continue_to_parse = true;
            while continue_to_parse {
                parser.cleanup_whitespaces();
                
                let param_expression = ExpressionParser::parse(parser);
                match param_expression {
                    Err(_) => return param_expression,
                    _ => ()
                };
                
                parser.cleanup_whitespaces();

                match parser.match_operator(&[BramaOperatorType::RightParentheses, BramaOperatorType::Comma]) {
                    Some(BramaOperatorType::RightParentheses) => continue_to_parse = false,  
                    Some(BramaOperatorType::Comma)            => {
                        if let Ok(BramaAstType::None) = param_expression {
                            parser.set_index(index_backup);
                            return Err(BramaError::SyntaxError)
                        }
                    },
                    _ => {
                        parser.set_index(index_backup);
                        return Err(BramaError::RightParanthesesMissing);
                    }
                }

                match param_expression {
                    Ok(BramaAstType::None) => (),
                    Ok(data) => arguments.push(Box::new(data)),
                    _ => (),
                };
            }

            parser.flags.set(inner_parser_flags);
            return Ok(BramaAstType::FuncCall {
                func_name_expression: ast,
                arguments,
                assign_to_temp: parser.flags.get().contains(SyntaxFlag::IN_EXPRESSION)
                                || parser.flags.get().contains(SyntaxFlag::IN_ASSIGNMENT)
                                || parser.flags.get().contains(SyntaxFlag::IN_FUNCTION_ARG)
                                || parser.flags.get().contains(SyntaxFlag::IN_RETURN)
            });
        }

        parser.flags.set(parser_flags);
        return Ok(BramaAstType::None);
    }
}