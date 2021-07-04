use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait, SyntaxFlag};
use crate::syntax::unary::UnaryParser;
use crate::syntax::util::update_functions_for_temp_return;
use crate::compiler::ast::KaramelAstType;
use crate::error::KaramelErrorType;

pub struct ModuloParser;
pub struct MultiplyDivideParser;
pub struct AddSubtractParser;

impl SyntaxParserTrait for ModuloParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        return parse_binary::<MultiplyDivideParser>(parser, &[KaramelOperatorType::Modulo]);
    }
}

impl SyntaxParserTrait for MultiplyDivideParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        return parse_binary::<UnaryParser>(parser, &[KaramelOperatorType::Multiplication, KaramelOperatorType::Division]);
    }
}

impl SyntaxParserTrait for AddSubtractParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        parse_binary::<ModuloParser>(parser, &[KaramelOperatorType::Addition, KaramelOperatorType::Subtraction])
    }
}

pub fn parse_binary<T: SyntaxParserTrait>(parser: &SyntaxParser, operators: &[KaramelOperatorType]) -> AstResult {
    let mut functions_updated_for_temp = false;
    let mut left_expr = T::parse(parser)?;
    match left_expr {
        KaramelAstType::None => return Ok(left_expr),
        _ => ()
    };

    loop {
        let index_backup = parser.get_index();
        parser.cleanup_whitespaces();
        
        if let Some(operator) = parser.match_operator(operators) {
            if !functions_updated_for_temp {
                update_functions_for_temp_return(&left_expr);
                functions_updated_for_temp = true;
            }
            
            parser.cleanup_whitespaces();
            let parser_flags  = parser.flags.get();
            parser.flags.set(parser_flags | SyntaxFlag::IN_EXPRESSION);
            
            let right_expr = T::parse(parser);
            match right_expr {
                Ok(KaramelAstType::None) => return Err(KaramelErrorType::RightSideOfExpressionNotFound),
                Ok(_) => (),
                Err(_) => return right_expr
            };

            parser.flags.set(parser_flags);
            left_expr = KaramelAstType::Binary {
                left: Box::new(left_expr),
                operator,
                right: Box::new(right_expr.unwrap())
            };
        }
        else {
            parser.set_index(index_backup);
            break;
        }
    }

    Ok(left_expr)
}
