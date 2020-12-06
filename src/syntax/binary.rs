use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::syntax::unary::UnaryParser;
use crate::compiler::ast::BramaAstType;

pub struct ModuloParser;
pub struct MultiplyDivideParser;
pub struct AddSubtractParser;

impl SyntaxParserTrait for ModuloParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        return parse_binary::<MultiplyDivideParser>(parser, &[BramaOperatorType::Modulo]);
    }
}

impl SyntaxParserTrait for MultiplyDivideParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        return parse_binary::<UnaryParser>(parser, &[BramaOperatorType::Multiplication, BramaOperatorType::Division]);
    }
}

impl SyntaxParserTrait for AddSubtractParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        return parse_binary::<ModuloParser>(parser, &[BramaOperatorType::Addition, BramaOperatorType::Subtraction]);
    }
}

pub fn parse_binary<T: SyntaxParserTrait>(parser: &SyntaxParser, operators: &[BramaOperatorType]) -> AstResult {
    let mut left_expr = T::parse(parser);
    match left_expr {
        Ok(BramaAstType::None) => return left_expr,
        Ok(_) => (),
        Err(_) => return left_expr
    };

    loop {
        parser.backup();
        parser.clear_whitespaces();
        if let Some(operator) = parser.match_operator(operators) {
            parser.clear_whitespaces();
            
            let right_expr = T::parse(parser);
            match right_expr {
                Ok(BramaAstType::None) => return Err(("Right side of expression not found", 0, 0)),
                Ok(_) => (),
                Err(_) => return right_expr
            };

            left_expr = Ok(BramaAstType::Binary {
                left: Box::new(left_expr.unwrap()),
                operator: operator,
                right: Box::new(right_expr.unwrap())
            });
        }
        else {
            parser.restore();
            break;
        }
    }

    return left_expr;
}
