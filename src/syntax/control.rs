use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::syntax::binary::AddSubtractParser;

pub struct ExpressionParser;
pub struct OrParser;
pub struct AndParser;
pub struct EqualityParser;
pub struct ControlParser;

impl SyntaxParserTrait for ExpressionParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        return OrParser::parse(parser);
    }
}

impl SyntaxParserTrait for OrParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        return parse_control::<AndParser>(parser, &[BramaOperatorType::Or]);
    }
}

impl SyntaxParserTrait for AndParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        return parse_control::<EqualityParser>(parser, &[BramaOperatorType::And]);
    }
}

impl SyntaxParserTrait for EqualityParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        return parse_control::<ControlParser>(parser, &[BramaOperatorType::Equal, BramaOperatorType::NotEqual]);
    }
}

impl SyntaxParserTrait for ControlParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        return parse_control::<AddSubtractParser>(parser, &[BramaOperatorType::GreaterEqualThan, 
            BramaOperatorType::GreaterThan,
            BramaOperatorType::LessEqualThan, 
            BramaOperatorType::LessThan]);
    }
}

pub fn parse_control<T: SyntaxParserTrait>(parser: &SyntaxParser, operators: &[BramaOperatorType]) -> AstResult {
    let mut left_expr = T::parse(parser);
    match left_expr {
        Ok(BramaAstType::None) => return left_expr,
        Ok(_) => (),
        Err(_) => return left_expr
    };
    
    loop {
        parser.clear_whitespaces();
        if let Some(operator) = parser.match_operator(operators) {
            parser.clear_whitespaces();
            
            let right_expr = T::parse(parser);
            match right_expr {
                Ok(BramaAstType::None) => return Err(("Right side of expression not found", 0, 0)),
                Ok(_) => (),
                Err(_) => return right_expr
            };

            left_expr = Ok(BramaAstType::Control {
                left: Box::new(left_expr.unwrap()),
                operator: operator,
                right: Box::new(right_expr.unwrap())
            });
        }        
        else {
            break;
        }
    }

    return left_expr;
}
