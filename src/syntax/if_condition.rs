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
            let true_body = match parser.get_newline() {
                (true, _) => MultiLineBlockParser::parse(parser),
                (false, _) => SingleLineBlockParser::parse(parser)
            }?;
            parser.set_indentation(indentation);

            if true_body == BramaAstType::None {
                return Err(("If condition body not found", 0, 0));
            }

            let mut else_body: Option<Box<BramaAstType>> = None;

            loop {
                parser.indentation_check()?;
                if parser.match_keyword(BramaKeywordType::Else) {
                    parser.clear_whitespaces();
                    if let None = parser.match_operator(&[BramaOperatorType::ColonMark]) {
                        return Err(("':' missing", 0, 0));
                    }
                    parser.clear_whitespaces();
                    
                    let body = match parser.get_newline() {
                        (true, size)  => {
                            parser.set_indentation(size);
                            MultiLineBlockParser::parse(parser)
                        },
                        (false, _) => SingleLineBlockParser::parse(parser)
                    }?;

                    if body == BramaAstType::None {
                        return Err(("If condition body not found", 0, 0));
                    }

                    else_body = Some(Box::new(body))
                }
                else {
                    break;
                }

                if let Err((_,_,_)) = parser.indentation_check() {
                    break;
                }
            }

            let assignment_ast = BramaAstType::IfStatement {
                condition: Box::new(expression.unwrap()),
                body: Box::new(true_body),
                else_body: else_body
            };

            println!("{:?}", assignment_ast);

            parser.set_indentation(indentation);
            return Ok(assignment_ast);
        }
        
        parser.restore();
        return Ok(BramaAstType::None);
    }
}
