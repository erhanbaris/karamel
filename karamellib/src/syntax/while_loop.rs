use std::rc::Rc;

use crate::syntax::util::map_parser;
use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait, SyntaxFlag};
use crate::compiler::ast::{KaramelAstType};
use crate::syntax::block::{SingleLineBlockParser, MultiLineBlockParser};
use crate::syntax::expression::ExpressionParser;
use crate::error::KaramelErrorType;

use super::assignment::AssignmentParser;
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum LoopType {
    Simple(Rc<KaramelAstType>),
    Scalar { 
        variable: Rc<KaramelAstType>, 
        control: Rc<KaramelAstType>,
        increment: Rc<KaramelAstType>
    },
    Endless
}

pub struct WhileLoopParser;

impl SyntaxParserTrait for WhileLoopParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        let index_backup = parser.get_index();
        parser.indentation_check()?;

        let indentation = parser.get_indentation();
        let loop_type = match parser.match_keywords(&[KaramelKeywordType::Endless, KaramelKeywordType::While]) {
            // Endless loop
            Some(KaramelKeywordType::Endless) => LoopType::Endless,

            // While loop
            Some(KaramelKeywordType::While) => {

                /* AssignmentParser has indentation check so we need to move indentation forward */
                parser.cleanup_whitespaces();

                /*

                We need to detect while loop type. We are checking body loop control section to understand while loop type.

                Parser: ExpressionParser::parse
                Example: 
                    döngü doğru:

                Parser: AssignmentParser::parse
                Example:
                    döngü 1 = 1, a <10, ++a:
                */
                let loop_expression = map_parser(parser, &[AssignmentParser::parse, ExpressionParser::parse])?;
                let loop_type = match &loop_expression {
                    KaramelAstType::None =>  {
                        /* Reset indentation */
                        parser.set_indentation(indentation);
                        return Ok(KaramelAstType::None);
                    },

                    // It is scalar loop
                    KaramelAstType::Assignment { variable: _, operator, expression: _ } => {
                        /* Loop just accept assignation operator, other operators are not valid */
                        if !operator.is_same(KaramelOperatorType::Assign) {
                            return Err(KaramelErrorType::AssignOperatorRequiredForLoop);
                        }

                        parser.cleanup_whitespaces();
                        if parser.match_operator(&[KaramelOperatorType::Comma]).is_none() {
                            return Err(KaramelErrorType::CommaIsMissing)
                        }

                        parser.cleanup_whitespaces();
                        let loop_control = ExpressionParser::parse(parser)?;

                        parser.cleanup_whitespaces();
                        if parser.match_operator(&[KaramelOperatorType::Comma]).is_none() {
                            return Err(KaramelErrorType::CommaIsMissing)
                        }

                        parser.cleanup_whitespaces();
                        let loop_increment = ExpressionParser::parse(parser)?;
                        parser.cleanup_whitespaces();

                        LoopType::Scalar {
                            variable: Rc::new(loop_expression),
                            control: Rc::new(loop_control),
                            increment: Rc::new(loop_increment)
                        }
                    },

                    // It is simple loop with condition
                    _ => LoopType::Simple(Rc::new(loop_expression.clone()))
                };

                loop_type
            },
            _ => {

                /* It is not a loop, parser need to continue */

                /* Reset indentation */
                parser.set_indentation(indentation);
                return Ok(KaramelAstType::None);
            }
        };

        parser.cleanup_whitespaces();
        if let None = parser.match_operator(&[KaramelOperatorType::ColonMark]) {
            return Err(KaramelErrorType::ColonMarkMissing);
        }

        parser.cleanup_whitespaces();
        let parser_flags  = parser.flags.get();
        parser.flags.set(parser_flags | SyntaxFlag::LOOP);

        let body = match parser.get_newline() {
            (true, _) => {
                parser.in_indication()?;
                MultiLineBlockParser::parse(parser)
            },
            (false, _) => SingleLineBlockParser::parse(parser)
        }?;

        /* Reset indentation and flag values */
        parser.set_indentation(indentation);
        parser.flags.set(parser_flags);

        return Ok(KaramelAstType::Loop {
            loop_type: loop_type,
            body: Rc::new(body)
        });
    }
}
