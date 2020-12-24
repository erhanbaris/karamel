use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::syntax::control::ExpressionParser;
use crate::compiler::ast::BramaAstType;

pub struct AssignmentParser;

impl SyntaxParserTrait for AssignmentParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        parser.backup();
        parser.indentation_check()?;
        let token = parser.peek_token();

        if token.is_ok() {
            if let BramaTokenType::Symbol(symbol) = &token.unwrap().token_type {
                parser.consume_token();
                parser.cleanup_whitespaces();

                if let Some(operator) = parser.match_operator(&[BramaOperatorType::Assign, 
                    BramaOperatorType::AssignAddition,
                    BramaOperatorType::AssignDivision,
                    BramaOperatorType::AssignMultiplication,
                    BramaOperatorType::AssignSubtraction]) {
                    parser.cleanup_whitespaces();
                    
                    let expression = ExpressionParser::parse(parser);
                    match expression {
                        Ok(BramaAstType::None) => return expression,
                        Ok(_) => (),
                        Err(_) => return expression
                    };

                    let assignment_ast = BramaAstType::Assignment {
                        variable: symbol.clone(),
                        operator: operator,
                        expression: Box::new(expression.unwrap())
                    };

                    return Ok(assignment_ast);
                }
            }
        }
        
        parser.restore();
        return Ok(BramaAstType::None);
    }
}
