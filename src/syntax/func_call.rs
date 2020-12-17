use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::syntax::control::ExpressionParser;
use crate::compiler::ast::BramaAstType;

pub struct FuncCallParser;

impl SyntaxParserTrait for FuncCallParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        parser.backup();
        parser.clear_whitespaces();
        let token = parser.peek_token();

        if token.is_ok() {
            if let BramaTokenType::Symbol(name) = &token.unwrap().token_type {
                parser.consume_token();
                let mut name_collection = Vec::new();
                name_collection.push(name.to_string());

                loop {
                    if let Some(_) = parser.match_operator(&[BramaOperatorType::ColonMark]) {
                        if let Some(_) = parser.match_operator(&[BramaOperatorType::ColonMark]) {
                            let token = parser.peek_token();
                            if let BramaTokenType::Symbol(name) = &token.unwrap().token_type {
                                parser.consume_token();
                                name_collection.push(name.to_string());
                            }
                            else {
                                return Err(("Function name required", 0, 0));
                            }
                        }
                        else {
                            return Err(("Colon mark required", 0, 0));
                        }
                    }
                    else {
                        break;
                    }
                }

                parser.clear_whitespaces();

                if let Some(_) = parser.match_operator(&[BramaOperatorType::LeftParentheses]) {
                    let mut arguments = Vec::new();

                    /* Parse function call arguments */
                    let mut continue_to_parse = true;
                    while continue_to_parse {
                        parser.clear_whitespaces();
                        
                        let expression = ExpressionParser::parse(parser);
                        match expression {
                            Err(_) => return expression,
                            _ => ()
                        };
                        
                        parser.clear_whitespaces();

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

                    let funccall_ast = BramaAstType::FuncCall {
                        names: name_collection.to_vec(),
                        arguments: arguments
                    };

                    return Ok(funccall_ast);
                }
            }
        }
        
        parser.restore();
        return Ok(BramaAstType::None);
    }
}