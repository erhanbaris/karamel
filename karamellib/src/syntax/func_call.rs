use std::cell::Cell;

use crate::types::*;
use crate::syntax::util::update_functions_for_temp_return;
use crate::syntax::{SyntaxParser, SyntaxParserTrait, SyntaxFlag, ExtensionSyntaxParser};
use crate::syntax::expression::ExpressionParser;
use crate::syntax::primative::PrimativeParser;
use crate::compiler::ast::KaramelAstType;
use crate::syntax::util::map_parser;
use crate::error::KaramelErrorType;

pub struct FuncCallParser;

impl SyntaxParserTrait for FuncCallParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        if parser.flags.get().contains(SyntaxFlag::IN_DICT_INDEXER) {
            return Ok(KaramelAstType::None);
        }

        let index = parser.get_index();
        parser.cleanup_whitespaces();
        let token = parser.peek_token();

        if token.is_ok() {
            let mut function_name = map_parser(parser, &[PrimativeParser::parse_module_path, PrimativeParser::parse_symbol])?;

            match &function_name {
                KaramelAstType::None => (),
                _ => { 
                    let parse_result = FuncCallParser::parse_suffix(&mut function_name, parser)?;
                    match parse_result {
                        KaramelAstType::None => (),
                        _ => return Ok(parse_result)
                    }; 
                }
            };
        }
        
        parser.set_index(index);
        return Ok(KaramelAstType::None);
    }
}

impl ExtensionSyntaxParser for FuncCallParser {
    fn parsable(parser: &SyntaxParser) -> bool {
        if parser.flags.get().contains(SyntaxFlag::IN_DICT_INDEXER) {
            return false;
        }

        parser.check_operator(&KaramelOperatorType::LeftParentheses)
    }

    fn parse_suffix(ast: &mut KaramelAstType, parser: &SyntaxParser) -> AstResult {

        let index_backup = parser.get_index();
        let parser_flags  = parser.flags.get();
        parser.cleanup_whitespaces();

        if let Some(_) = parser.match_operator(&[KaramelOperatorType::LeftParentheses]) {
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

                match parser.match_operator(&[KaramelOperatorType::RightParentheses, KaramelOperatorType::Comma]) {
                    Some(KaramelOperatorType::RightParentheses) => continue_to_parse = false,  
                    Some(KaramelOperatorType::Comma)            => {
                        if let Ok(KaramelAstType::None) = param_expression {
                            parser.set_index(index_backup);
                            return Err(KaramelErrorType::SyntaxError)
                        }
                    },
                    _ => {
                        return Err(KaramelErrorType::RightParanthesesMissing);
                    }
                }

                match param_expression {
                    Ok(KaramelAstType::None) => (),
                    Ok(data) => arguments.push(Box::new(data)),
                    _ => (),
                };
            }

            parser.flags.set(inner_parser_flags);
            return Ok(KaramelAstType::FuncCall {
                func_name_expression: Box::new(ast.clone()),
                arguments,
                assign_to_temp: Cell::new(parser.flags.get().contains(SyntaxFlag::IN_EXPRESSION)
                                || parser.flags.get().contains(SyntaxFlag::IN_ASSIGNMENT)
                                || parser.flags.get().contains(SyntaxFlag::IN_FUNCTION_ARG)
                                || parser.flags.get().contains(SyntaxFlag::IN_RETURN))
            });
        }
        /* parse for 'object.method()' */
        else if let Some(_) = parser.match_operator(&[KaramelOperatorType::Dot]) {
            

            let sub_ast = FuncCallParser::parse(parser)?;

            return match &sub_ast {
                KaramelAstType::FuncCall {
                    func_name_expression,
                    arguments: _,
                    assign_to_temp: _ 
                } => {
                    match &**func_name_expression {
                        KaramelAstType::Symbol(_) => {
                            update_functions_for_temp_return(ast);
                            Ok(KaramelAstType::AccessorFuncCall {
                                source: Box::new(ast.clone()),
                                indexer: Box::new(sub_ast),
                                assign_to_temp: Cell::new(true)
                            })
                        },
                        _ => {
                            log::debug!("Function call syntax not valid {:?}", func_name_expression);
                            Err(KaramelErrorType::FunctionCallSyntaxNotValid)
                        }
                    }
                }
                _ => Ok(sub_ast)
            };
        }

        parser.flags.set(parser_flags);
        return Ok(KaramelAstType::None);
    }
}