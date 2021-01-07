use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait, SyntaxFlag};
use crate::syntax::control::ExpressionParser;
use crate::compiler::ast::BramaAstType;

pub struct FuncCallParser;

impl SyntaxParserTrait for FuncCallParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        let index_backup = parser.get_index();
        parser.cleanup_whitespaces();
        let token = parser.peek_token();
        let mut is_function = false;

        if token.is_ok() {

            let parser_flags  = parser.flags.get();
            let mut call_expression: AstResult = Ok(BramaAstType::None);
            let mut last_index_backup = parser.get_index();

            loop {
                parser.flags.set(parser_flags | SyntaxFlag::SKIP_FUNC_CALL);
                let current_expression = ExpressionParser::parse(parser);

                println!("1. {:?}", current_expression);
                println!("2. {:?}", call_expression);
                println!("3. {:?}", parser.peek_token());

                call_expression = match current_expression {
                    Ok(BramaAstType::None) => call_expression,
                    Err(_) => {
                        match call_expression {
                            Ok(BramaAstType::None) => return current_expression,
                            Ok(_) => call_expression,
                            _ => return current_expression
                        }
                    },
                    _ => current_expression
                };

                if let Some(_) = parser.match_operator(&[BramaOperatorType::LeftParentheses]) {
                    is_function = true;
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
                                    return Err(("Syntax error, undefined syntax", 0, 0))
                                }
                                else {()}
                            },
                            _ => {
                                parser.set_index(index_backup);
                                return Err(("Right parantheses missing", 0, 0));
                            }
                        }

                        match param_expression {
                            Ok(BramaAstType::None) => (),
                            Ok(data) => arguments.push(Box::new(data)),
                            _ => (),
                        };
                    }

                    /* Last index */
                    last_index_backup = parser.get_index();

                    println!("#### {:?}", call_expression);

                    parser.flags.set(inner_parser_flags);
                    call_expression = Ok(BramaAstType::FuncCall {
                        func_name_expression: Box::new(call_expression.unwrap()),
                        arguments: arguments,
                        assign_to_temp: parser.flags.get().contains(SyntaxFlag::IN_EXPRESSION)
                                     || parser.flags.get().contains(SyntaxFlag::IN_ASSIGNMENT)
                                     || parser.flags.get().contains(SyntaxFlag::IN_FUNCTION_ARG)
                                     || parser.flags.get().contains(SyntaxFlag::IN_RETURN)
                    });
                }
                else {
                    println!("Reseted");
                    parser.set_index(last_index_backup);
                    break;
                }
            }

            parser.flags.set(parser_flags);
            if is_function {
                println!("{:?}", call_expression);
                return call_expression;
            }
        }
        
        parser.set_index(index_backup);
        return Ok(BramaAstType::None);
    }
}

impl FuncCallParser {
    pub fn can_be_func_call(parser: &SyntaxParser) -> bool {
        parser.check_operator(&BramaOperatorType::LeftParentheses)
    }

    pub fn func_call_parse(ast: Box<BramaAstType>, parser: &SyntaxParser) -> AstResult {
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
                            return Err(("Syntax error, undefined syntax", 0, 0))
                        }
                        else {()}
                    },
                    _ => {
                        parser.set_index(index_backup);
                        return Err(("Right parantheses missing", 0, 0));
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
                arguments: arguments,
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