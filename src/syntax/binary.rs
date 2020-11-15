use crate::types::*;
use crate::syntax::SyntaxParser;
use crate::syntax::unary::UnaryParser;

pub struct ModuloParser;
pub struct MultiplyDivideParser;
pub struct AddSubtractParser;

/*
pub struct BitwiseShiftParser;
pub struct BitwiseOrParser;
pub struct BitwiseAndParser;
pub struct BitwiseXorParser;


impl SyntaxParserTrait for BitwiseOrParser {
    type Item = BitwiseOrParser;

    fn parse(parser: &SyntaxParser) -> AstResult {
        return parse_binary::<BitwiseXorParser>(parser, &[BramaOperatorType::BitwiseOr]);
    }
}

impl SyntaxParserTrait for BitwiseXorParser {
    type Item = BitwiseXorParser;

    fn parse(parser: &SyntaxParser) -> AstResult {
        return parse_binary::<BitwiseAndParser>(parser, &[BramaOperatorType::BitwiseXor]);
    }
}

impl SyntaxParserTrait for BitwiseAndParser {
    type Item = BitwiseAndParser;

    fn parse(parser: &SyntaxParser) -> AstResult {
        return parse_binary::<EqualityParser>(parser, &[BramaOperatorType::BitwiseAnd]);
    }
}

impl SyntaxParserTrait for BitwiseShiftParser {
    type Item = BitwiseShiftParser;

    fn parse(parser: &SyntaxParser) -> AstResult {
        return parse_binary::<AddSubtractParser>(parser, &[BramaOperatorType::BitwiseLeftShift, BramaOperatorType::BitwiseRightShift]);
    }
}
*/


impl SyntaxParserTrait for ModuloParser {
    type Item = ModuloParser;

    fn parse(parser: &SyntaxParser) -> AstResult {
        return parse_binary::<MultiplyDivideParser>(parser, &[BramaOperatorType::Modulo]);
    }
}

impl SyntaxParserTrait for MultiplyDivideParser {
    type Item = MultiplyDivideParser;

    fn parse(parser: &SyntaxParser) -> AstResult {
        return parse_binary::<UnaryParser>(parser, &[BramaOperatorType::Multiplication, BramaOperatorType::Division]);
    }
}

impl SyntaxParserTrait for AddSubtractParser {
    type Item = AddSubtractParser;

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
            break;
        }
    }

    return left_expr;
}
