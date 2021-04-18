use crate::types::*;
use crate::syntax::util::update_functions_for_temp_return;
use crate::syntax::{SyntaxParser, SyntaxParserTrait, SyntaxFlag, ExtensionSyntaxParser};
use crate::syntax::expression::ExpressionParser;
use crate::syntax::primative::PrimativeParser;
use crate::compiler::ast::BramaAstType;
use crate::syntax::util::map_parser;
use crate::error::BramaErrorType;

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
            let mut function_name = map_parser(parser, &[PrimativeParser::parse_function_map, PrimativeParser::parse_symbol])?;

            match &function_name {
                BramaAstType::None => (),
                _ => { 
                    let parse_result = FuncCallParser::parse_suffix(&mut function_name, parser)?;
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

    fn parse_suffix(ast: &mut BramaAstType, parser: &SyntaxParser) -> AstResult {

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
                            return Err(BramaErrorType::SyntaxError)
                        }
                    },
                    _ => {
                        parser.set_index(index_backup);
                        return Err(BramaErrorType::RightParanthesesMissing);
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
                func_name_expression: Box::new(ast.clone()),
                arguments,
                assign_to_temp: parser.flags.get().contains(SyntaxFlag::IN_EXPRESSION)
                                || parser.flags.get().contains(SyntaxFlag::IN_ASSIGNMENT)
                                || parser.flags.get().contains(SyntaxFlag::IN_FUNCTION_ARG)
                                || parser.flags.get().contains(SyntaxFlag::IN_RETURN)
            });
        }
        /* parse for 'object.method()' */
        else if let Some(_) = parser.match_operator(&[BramaOperatorType::Dot]) {
            

            let sub_ast = FuncCallParser::parse(parser)?;

            return match &sub_ast {
                BramaAstType::FuncCall {
                    func_name_expression,
                    arguments: _,
                    assign_to_temp: _ 
                } => {
                    match &**func_name_expression {
                        BramaAstType::Symbol(_) => {
                            update_functions_for_temp_return(ast);
                            Ok(BramaAstType::AccessorFuncCall {
                                source: Box::new(ast.clone()),
                                indexer: Box::new(sub_ast),
                                assign_to_temp: true
                            })
                        },
                        _ => {
                            log::debug!("Function call syntax not valid {:?}", func_name_expression);
                            Err(BramaErrorType::FunctionCallSyntaxNotValid)
                        }
                    }
                }
                _ => Ok(sub_ast)
            };
        }

        parser.flags.set(parser_flags);
        return Ok(BramaAstType::None);
    }
}