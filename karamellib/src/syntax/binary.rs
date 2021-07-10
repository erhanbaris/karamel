use std::rc::Rc;

use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait, SyntaxFlag};
use crate::syntax::unary::UnaryParser;
use crate::syntax::util::update_functions_for_temp_return;
use crate::compiler::ast::KaramelAstType;
use crate::error::KaramelErrorType;

use super::util::with_flag;

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
            
            let right_expr = with_flag(SyntaxFlag::IN_EXPRESSION, parser, || T::parse(parser));
            match right_expr {
                Ok(KaramelAstType::None) => return Err(KaramelErrorType::RightSideOfExpressionNotFound),
                Ok(_) => (),
                Err(_) => return right_expr
            };

            left_expr = KaramelAstType::Binary {
                left: Rc::new(left_expr),
                operator,
                right: Rc::new(right_expr.unwrap())
            };
        }
        else {
            parser.set_index(index_backup);
            break;
        }
    }

    Ok(left_expr)
}
