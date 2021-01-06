use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait, SyntaxFlag};
use crate::syntax::control::ExpressionParser;
use crate::compiler::ast::BramaAstType;

pub struct FuncCallParser;

impl SyntaxParserTrait for FuncCallParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        parser.backup();
        parser.cleanup_whitespaces();
        let token = parser.peek_token();

        if token.is_ok() {

            let parser_flags  = parser.flags.get();
            let mut call_expression: AstResult = Ok(BramaAstType::None);

            loop {
                parser.flags.set(parser_flags | SyntaxFlag::SKIP_FUNC_CALL);
                let current_expression = ExpressionParser::parse(parser);
                println!("{:?}", call_expression);
                call_expression = match current_expression {
                    Err(_) => {
                        match call_expression {
                            Ok(BramaAstType::None) => return current_expression,
                            Ok(_) => call_expression,
                            _ => return current_expression
                        }
                    },
                    _ => current_expression
                };

                let mut name_collection = Vec::new();

                loop {
                    if let Some(_) = parser.match_operator(&[BramaOperatorType::ColonMark]) {
                        if let Some(_) = parser.match_operator(&[BramaOperatorType::ColonMark]) {
                            let token = parser.peek_token();
                            if let BramaTokenType::Symbol(name) = &token.unwrap().token_type {
                                parser.consume_token();
                            }
                            else {
                                return Err(("Function name required", 0, 0));
                            }
                        }
                        else {
                            break;
                        }
                    }
                    else {
                        break;
                    }
                }

                parser.cleanup_whitespaces();

                if let Some(_) = parser.match_operator(&[BramaOperatorType::LeftParentheses]) {
                    let mut arguments = Vec::new();

                    let parser_flags  = parser.flags.get();
                    parser.flags.set(parser_flags | SyntaxFlag::IN_FUNCTION_ARG);

                    /* Parse function call arguments */
                    let mut continue_to_parse = true;
                    while continue_to_parse {
                        parser.cleanup_whitespaces();
                        
                        let expression = ExpressionParser::parse(parser);
                        match expression {
                            Err(_) => return expression,
                            _ => ()
                        };
                        
                        parser.cleanup_whitespaces();

                        match parser.match_operator(&[BramaOperatorType::RightParentheses, BramaOperatorType::Comma]) {
                            Some(BramaOperatorType::RightParentheses) => continue_to_parse = false,  
                            Some(BramaOperatorType::Comma)            => {
                                if let Ok(BramaAstType::None) = expression {
                                    return Err(("Syntax error, undefined syntax", 0, 0))
                                }
                                else {()}
                            },
                            _ => return Err(("Right parantheses missing", 0, 0))
                        }

                        match expression {
                            Ok(BramaAstType::None) => (),
                            Ok(data) => arguments.push(Box::new(data)),
                            _ => (),
                        };
                    }

                    parser.flags.set(parser_flags);
                    call_expression = Ok(BramaAstType::FuncCall {
                        names: name_collection.to_vec(),
                        arguments: arguments,
                        assign_to_temp: parser.flags.get().contains(SyntaxFlag::IN_EXPRESSION)
                                     || parser.flags.get().contains(SyntaxFlag::IN_ASSIGNMENT)
                                     || parser.flags.get().contains(SyntaxFlag::IN_FUNCTION_ARG)
                                     || parser.flags.get().contains(SyntaxFlag::IN_RETURN)
                    });
                }
                else {
                    break;
                }
            }

            parser.flags.set(parser_flags);
            return call_expression;
        }
        
        parser.restore();
        return Ok(BramaAstType::None);
    }
}