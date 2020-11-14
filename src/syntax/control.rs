use crate::types::*;
use crate::syntax::SyntaxParser;
use crate::syntax::binary::AddSubtractParser;

pub struct OrParser;
pub struct AndParser;
pub struct EqualityParser;
pub struct ControlParser;

impl SyntaxParserTrait for OrParser {
    type Item = OrParser;

    fn parse(parser: &SyntaxParser) -> AstResult {
        return parse_control_keyword::<AndParser>(parser, &[BramaKeywordType::Or]);
    }
}

impl SyntaxParserTrait for AndParser {
    type Item = AndParser;

    fn parse(parser: &SyntaxParser) -> AstResult {
        return parse_control_keyword::<EqualityParser>(parser, &[BramaKeywordType::And]);
    }
}

impl SyntaxParserTrait for EqualityParser {
    type Item = EqualityParser;

    fn parse(parser: &SyntaxParser) -> AstResult {
        return parse_control::<ControlParser>(parser, &[BramaOperatorType::Equal, BramaOperatorType::NotEqual]);
    }
}

impl SyntaxParserTrait for ControlParser {
    type Item = ControlParser;

    fn parse(parser: &SyntaxParser) -> AstResult {
        return parse_control::<AddSubtractParser>(parser, &[BramaOperatorType::GreaterEqualThan, 
            BramaOperatorType::GreaterThan,
            BramaOperatorType::LessEqualThan, 
            BramaOperatorType::LessThan]);
    }
}

pub fn parse_control<T: SyntaxParserTrait>(parser: &SyntaxParser, operators: &[BramaOperatorType]) -> AstResult {
    let left_expr = T::parse(parser);
    match left_expr {
        Ok(BramaAstType::None) => return left_expr,
        Ok(_) => (),
        Err(_) => return left_expr
    };
    
    parser.clear_whitespaces();
    if let Some(operator) = parser.match_operator(operators) {
        parser.clear_whitespaces();
        
        let right_expr = T::parse(parser);
        match right_expr {
            Ok(BramaAstType::None) => return Err(("Right side of expression not found", 0, 0)),
            Ok(_) => (),
            Err(_) => return right_expr
        };

        return Ok(BramaAstType::Control {
            left: Box::new(left_expr.unwrap()),
            operator: operator,
            right: Box::new(right_expr.unwrap())
        });
    }

    return left_expr;
}

pub fn parse_control_keyword<T: SyntaxParserTrait>(parser: &SyntaxParser, keywords: &[BramaKeywordType]) -> AstResult {
    let left_expr = T::parse(parser);
    match left_expr {
        Ok(BramaAstType::None) => return left_expr,
        Ok(_) => (),
        Err(_) => return left_expr
    };
    
    parser.clear_whitespaces();
    if let Some(keyword) = parser.match_keyword(keywords) {
        parser.clear_whitespaces();
        
        let right_expr = T::parse(parser);
        match right_expr {
            Ok(BramaAstType::None) => return Err(("Right side of expression not found", 0, 0)),
            Ok(_) => (),
            Err(_) => return right_expr
        };

        return Ok(BramaAstType::Control {
            left: Box::new(left_expr.unwrap()),
            operator: keyword.to_operator(),
            right: Box::new(right_expr.unwrap())
        });
    }

    return left_expr;
}