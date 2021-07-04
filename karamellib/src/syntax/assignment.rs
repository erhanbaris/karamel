use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait, SyntaxFlag};
use crate::syntax::expression::ExpressionParser;
use crate::compiler::ast::KaramelAstType;

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

            let parser_flags  = parser.flags.get();
            parser.flags.set(parser_flags | SyntaxFlag::IN_ASSIGNMENT);
            
            let expression = ExpressionParser::parse(parser);
            match expression {
                Ok(KaramelAstType::None) => return expression,
                Ok(_) => (),
                Err(_) => return expression
            };

            parser.flags.set(parser_flags);

            let assignment_ast = KaramelAstType::Assignment {
                variable: Box::new(variable),
                operator,
                expression: Box::new(expression.unwrap())
            };

            return Ok(assignment_ast);
        }
        parser.set_index(index_backup);
        return Ok(KaramelAstType::None);
    }
}
