use std::rc::Rc;

use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait, SyntaxFlag};
use crate::compiler::ast::{KaramelAstType};
use crate::syntax::block::{SingleLineBlockParser, MultiLineBlockParser};
use crate::syntax::expression::ExpressionParser;
use crate::error::KaramelErrorType;

use super::assignment::AssignmentParser;
use super::util::{map_parser_with_flag, with_flag};
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
                let loop_expression = map_parser_with_flag(SyntaxFlag::IN_EXPRESSION, parser, &[AssignmentParser::parse, ExpressionParser::parse])?;
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

                        let loop_control = with_flag(SyntaxFlag::IN_EXPRESSION, parser, || ExpressionParser::parse(parser))?;
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


#[cfg(test)]
mod tests {
    use crate::error::{KaramelError, KaramelErrorType};

    use crate::types::*;
    use crate::parser::*;
    use crate::syntax::*;
    use crate::compiler::value::KaramelPrimative;
    use crate::compiler::ast::KaramelAstType;
    use crate::syntax::loops::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[warn(unused_macros)]
    macro_rules! test_compare {
        ($name:ident, $text:expr, $result:expr) => {
            #[test]
            fn $name () {
                let mut parser = Parser::new($text);
                match parser.parse() {
                    Err(_) => assert_eq!(true, false),
                    _ => ()
                };

                let syntax = SyntaxParser::new(parser.tokens().to_vec());
                assert_eq!(syntax.parse(), $result);
            }
        };
    }

    test_compare!(endless_1, r#"sonsuz:
    a = 1
"#, Ok(Rc::new(KaramelAstType::Loop {
    loop_type: LoopType::Endless,
        body: Rc::new(KaramelAstType::Assignment {
            variable: Rc::new(KaramelAstType::Symbol("a".to_string())),
            operator: KaramelOperatorType::Assign,
            expression: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1.0))))
        })
    } )));

    test_compare!(endless_2, r#"sonsuz:
    sonsuz:
        a = 1
"#, Ok(Rc::new(KaramelAstType::Loop {
    loop_type: LoopType::Endless,
        body: Rc::new(KaramelAstType::Loop {
            loop_type: LoopType::Endless,
                body: Rc::new(KaramelAstType::Assignment {
                    variable: Rc::new(KaramelAstType::Symbol("a".to_string())),
                    operator: KaramelOperatorType::Assign,
                    expression: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1.0))))
                })
            } )
    } )));

    test_compare!(simple_1, r#"döngü a == 1:
        a = 1
"#, Ok(Rc::new(KaramelAstType::Loop {
    loop_type: LoopType::Simple(Rc::new(KaramelAstType::Control {
        left: Rc::new(KaramelAstType::Symbol("a".to_string())),
        operator: KaramelOperatorType::Equal, 
        right: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1.0))))
    })),
        body: Rc::new(KaramelAstType::Assignment {
            variable: Rc::new(KaramelAstType::Symbol("a".to_string())),
            operator: KaramelOperatorType::Assign,
            expression: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1.0))))
        })
    } )));

    test_compare!(simple_2, r#"döngü test():
        a = 1
"#, Ok(Rc::new(KaramelAstType::Loop {
    loop_type: LoopType::Simple(Rc::new(KaramelAstType::FuncCall {
        func_name_expression: Rc::new(KaramelAstType::Symbol("test".to_string())),
        arguments: Vec::new(),
        assign_to_temp: Cell::new(true)
    })),
        body: Rc::new(KaramelAstType::Assignment {
            variable: Rc::new(KaramelAstType::Symbol("a".to_string())),
            operator: KaramelOperatorType::Assign,
            expression: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1.0))))
        })
    } )));

    test_compare!(simple_3, r#"döngü doğru:
        a = 1
"#, Ok(Rc::new(KaramelAstType::Loop {
    loop_type: LoopType::Simple(Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Bool(true))) )),
        body: Rc::new(KaramelAstType::Assignment {
            variable: Rc::new(KaramelAstType::Symbol("a".to_string())),
            operator: KaramelOperatorType::Assign,
            expression: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1.0))))
        })
    } )));

    test_compare!(scalar_1, r#"döngü i = 1, i < 2, ++i:
    doğru
"#, Ok(Rc::new(KaramelAstType::Loop {
        loop_type: LoopType::Scalar {
            variable: Rc::new(KaramelAstType::Assignment {
                variable: Rc::new(KaramelAstType::Symbol("i".to_string())),
                operator: KaramelOperatorType::Assign,
                expression: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1.0))))
            }), 
            control: Rc::new(KaramelAstType::Control {
                left: Rc::new(KaramelAstType::Symbol("i".to_string())), 
                operator: KaramelOperatorType::LessThan, 
                right: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(2.0))))
            }),
            increment: Rc::new(KaramelAstType::PrefixUnary {
                operator: KaramelOperatorType::Increment, 
                expression: Rc::new(KaramelAstType::Symbol("i".to_string())), 
                assign_to_temp: Cell::new(false)
            })
        },
        body: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Bool(true))))
    } )));

    test_compare!(scalar_2, r#"döngü i = 1:
    doğru
"#, Err(KaramelError::new(0, 12, KaramelErrorType::CommaIsMissing)));
test_compare!(scalar_3, r#"döngü i = 1, i < 1:
doğru
"#, Err(KaramelError::new(0, 19, KaramelErrorType::CommaIsMissing)));
test_compare!(scalar_4, r#"döngü i = 1, i < 1, :
doğru
"#, Err(KaramelError::new(0, 21, KaramelErrorType::IndentationIssue)));
test_compare!(scalar_5, r#"döngü i = 1, i < 1,
doğru
"#, Err(KaramelError::new(0, 19, KaramelErrorType::ColonMarkMissing)));
}
