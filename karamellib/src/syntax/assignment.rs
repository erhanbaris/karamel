use std::rc::Rc;

use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait, SyntaxFlag};
use crate::syntax::expression::ExpressionParser;
use crate::compiler::ast::KaramelAstType;

use super::util::with_flag;

pub struct AssignmentParser;

impl SyntaxParserTrait for AssignmentParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        let index_backup = parser.get_index();
        parser.indentation_check()?;

        let variable = ExpressionParser::parse(parser)?;

        match variable {
            KaramelAstType::Symbol(_) => (),
            KaramelAstType::Indexer{ body: _, indexer: _ } => (),
            _ =>  {
                parser.set_index(index_backup);
                return Ok(KaramelAstType::None);
            }
        };

        parser.cleanup_whitespaces();

        if let Some(operator) = parser.match_operator(&[KaramelOperatorType::Assign, 
            KaramelOperatorType::AssignAddition,
            KaramelOperatorType::AssignDivision,
            KaramelOperatorType::AssignMultiplication,
            KaramelOperatorType::AssignSubtraction]) {
            parser.cleanup_whitespaces();

            let expression = with_flag(SyntaxFlag::IN_ASSIGNMENT, parser, || ExpressionParser::parse(parser));            
            match expression {
                Ok(KaramelAstType::None) => return expression,
                Ok(_) => (),
                Err(_) => return expression
            };

            let assignment_ast = KaramelAstType::Assignment {
                variable: Rc::new(variable),
                operator,
                expression: Rc::new(expression.unwrap())
            };

            return Ok(assignment_ast);
        }
        parser.set_index(index_backup);
        return Ok(KaramelAstType::None);
    }
}
