use crate::types::*;
use crate::syntax::SyntaxParser;
use crate::syntax::control::ExpressionParser;

pub struct AssignmentParser;

impl SyntaxParserTrait for AssignmentParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        parser.backup();
        parser.clear_whitespaces();
        let token = parser.peek_token();

        if let BramaTokenType::Symbol(symbol) = &token.unwrap().token_type {
            parser.consume_token();
            parser.clear_whitespaces();

            if let Some(operator) = parser.match_operator(&[BramaOperatorType::Assign, 
                BramaOperatorType::AssignAddition,
                BramaOperatorType::AssignDivision,
                BramaOperatorType::AssignMultiplication,
                BramaOperatorType::AssignSubtraction]) {
                parser.clear_whitespaces();
                
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

        parser.restore();
        return Ok(BramaAstType::None);
    }
}
