use std::rc::Rc;

use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait, SyntaxFlag};
use crate::syntax::binary::AddSubtractParser;
use crate::syntax::util::update_functions_for_temp_return;
use crate::compiler::ast::KaramelAstType;
use crate::error::KaramelErrorType;

use super::util::with_flag;

pub struct OrParser;
pub struct AndParser;
pub struct EqualityParser;
pub struct ControlParser;

impl<'a> SyntaxParserTrait<'a> for OrParser {
    fn parse(parser: &SyntaxParser<'a>) -> AstResult<'a> {
        parse_control::<AndParser>(parser, &[KaramelOperatorType::Or])
    }
}

impl<'a> SyntaxParserTrait<'a> for AndParser {
    fn parse(parser: &SyntaxParser<'a>) -> AstResult<'a> {
        parse_control::<EqualityParser>(parser, &[KaramelOperatorType::And])
    }
}

impl<'a> SyntaxParserTrait<'a> for EqualityParser {
    fn parse(parser: &SyntaxParser<'a>) -> AstResult<'a> {
        parse_control::<ControlParser>(parser, &[KaramelOperatorType::Equal, KaramelOperatorType::NotEqual])
    }
}

impl<'a> SyntaxParserTrait<'a> for ControlParser {
    fn parse(parser: &SyntaxParser<'a>) -> AstResult<'a> {
        special_control(parser)
    }
}

pub fn special_control<'a>(parser: &SyntaxParser<'a>) -> AstResult<'a> {
    let mut functions_updated_for_temp = false;
    let mut left_expr = AddSubtractParser::parse(parser)?;
    let operators = [KaramelOperatorType::GreaterEqualThan, 
        KaramelOperatorType::GreaterThan,
        KaramelOperatorType::LessEqualThan, 
        KaramelOperatorType::LessThan];
    match left_expr {
        KaramelAstType::None => return Ok(left_expr),
        _ => ()
    };
    
    loop {
        let index_backup = parser.get_index();
        parser.cleanup_whitespaces();
        if let Some(operator) = parser.match_operator(&operators) {
            if !functions_updated_for_temp {
                update_functions_for_temp_return(&left_expr);
                functions_updated_for_temp = true;
            }

            parser.cleanup_whitespaces();
            let right_expr = with_flag(SyntaxFlag::IN_EXPRESSION, parser, || AddSubtractParser::parse(parser));
            match right_expr {
                Ok(KaramelAstType::None) => return Err(KaramelErrorType::RightSideOfExpressionNotFound),
                Ok(_) => (),
                Err(_) => return right_expr
            };

            left_expr = match operator {
                KaramelOperatorType::LessEqualThan => KaramelAstType::Control {
                    left: Rc::new(right_expr.unwrap()),
                    operator: KaramelOperatorType::GreaterEqualThan,
                    right: Rc::new(left_expr)
                },
                KaramelOperatorType::LessThan => KaramelAstType::Control {
                    left: Rc::new(right_expr.unwrap()),
                    operator: KaramelOperatorType::GreaterThan,
                    right: Rc::new(left_expr)
                },
                _ => KaramelAstType::Control {
                    left: Rc::new(left_expr),
                    operator,
                    right: Rc::new(right_expr.unwrap())
                }
            };
        }        
        else {
            parser.set_index(index_backup);
            break;
        }
    }

    Ok(left_expr)
}

pub fn parse_control<'a, T: SyntaxParserTrait<'a>>(parser: &SyntaxParser<'a>, operators: &[KaramelOperatorType]) -> AstResult<'a> {
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

            left_expr = KaramelAstType::Control {
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
