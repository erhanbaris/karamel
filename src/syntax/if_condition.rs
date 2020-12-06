use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::syntax::control::ExpressionParser;
use crate::compiler::ast::BramaAstType;
use crate::syntax::block::{SingleLineBlockParser, MultiLineBlockParser};

pub struct IfConditiontParser;

impl SyntaxParserTrait for IfConditiontParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        parser.backup();
        parser.indentation_check()?;

        if parser.match_keyword(BramaKeywordType::If) {
            parser.clear_whitespaces();

            let expression = ExpressionParser::parse(parser);
            match expression {
                Ok(BramaAstType::None) => return expression,
                Ok(_) => (),
                Err(_) => return expression
            };

            if let None = parser.match_operator(&[BramaOperatorType::ColonMark]) {
                return Err(("':' missing", 0, 0));
            }

            parser.clear_whitespaces();
            let indentation = parser.get_indentation();
            parser.indentation_setable();
            let body = match parser.is_newline() {
                true  => MultiLineBlockParser::parse(parser),
                false => SingleLineBlockParser::parse(parser)
            }?;
            parser.set_indentation(indentation);

            if body == BramaAstType::None {
                return Err(("If condition body not found", 0, 0));
            }

            let assignment_ast = BramaAstType::IfStatement {
                condition: Box::new(expression.unwrap()),
                body: Box::new(body)
            };

            loop {
                if parser.match_keyword(BramaKeywordType::Else) {
                    parser.clear_whitespaces();

                    let expression = ExpressionParser::parse(parser);
                    match expression {
                        Ok(BramaAstType::None) => return expression,
                        Ok(_) => (),
                        Err(_) => return expression
                    };

                    if let None = parser.match_operator(&[BramaOperatorType::ColonMark]) {
                        return Err(("':' missing", 0, 0));
                    }
                }
                else {
                    break;
                }
            }

            println!("{:?}", assignment_ast);
            return Ok(assignment_ast);
        }
        
        parser.restore();
        return Ok(BramaAstType::None);
    }
}
