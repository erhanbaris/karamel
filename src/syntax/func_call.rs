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
                parser.clear_whitespaces();

                if let Some(_) = parser.match_operator(&[BramaOperatorType::LeftParentheses]) {
                    parser.clear_whitespaces();
                    
                    let expression = ExpressionParser::parse(parser);
                    match expression {
                        Ok(BramaAstType::None) => return expression,
                        Ok(_) => (),
                        Err(_) => return expression
                    };
                    
                    parser.clear_whitespaces();
                    if let Some(_) = parser.match_operator(&[BramaOperatorType::RightParentheses]) {
                        
                        let funcall_ast = BramaAstType::FunCall {
                            name: name.to_string(),
                            expression: Box::new(expression.unwrap())
                        };
                        return Ok(funcall_ast);
                    }
                    else {
                        return Err(("Right parantheses missing", 0, 0));
                    }
                }
            }
        }
        
        parser.restore();
        return Ok(BramaAstType::None);
    }
}