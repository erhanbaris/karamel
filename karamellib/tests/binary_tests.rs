extern crate karamellib;

#[cfg(test)]
mod tests {
    use karamellib::error::{BramaError, BramaErrorType};

    use crate::karamellib::parser::*;
    use crate::karamellib::types::*;
    use crate::karamellib::syntax::*;
    use crate::karamellib::compiler::value::BramaPrimative;
    use crate::karamellib::compiler::ast::BramaAstType;
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

    test_compare!(add_subtract_1, "10 + 10", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0)))), 
        operator: BramaOperatorType::Addition, 
        right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0))))
    }));

    test_compare!(add_subtract_2, "10 - 10", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0)))), 
        operator: BramaOperatorType::Subtraction, 
        right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0))))
    }));

    test_compare!(add_subtract_3, "5 * 2 mod 2 - 10", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::Binary {
            left: Box::new(BramaAstType::Binary {
                left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(5.0)))),
                operator: BramaOperatorType::Multiplication, 
                right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(2.0))))
            }),
            operator: BramaOperatorType::Modulo, 
            right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(2.0))))
        }), 
        operator: BramaOperatorType::Subtraction, 
        right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0))))
    }));

    test_compare!(add_subtract_4, "22 + 5 * 2 mod 2", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(22.0)))), 
        operator: BramaOperatorType::Addition, 
        right: Box::new(BramaAstType::Binary {
            left: Box::new(BramaAstType::Binary {
                left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(5.0)))),
                operator: BramaOperatorType::Multiplication, 
                right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(2.0))))
            }),
            operator: BramaOperatorType::Modulo, 
            right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(2.0))))
        })
    }));

    test_compare!(add_subtract_5, "11 + 12 + 13", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::Binary {
            left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(11.0)))), 
            operator: BramaOperatorType::Addition, 
            right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(12.0))))
        }), 
        operator: BramaOperatorType::Addition, 
        right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(13.0))))
    }));

    test_compare!(multiply_divide_1, "10*10", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0)))), 
        operator: BramaOperatorType::Multiplication, 
        right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0))))
    }));

    test_compare!(multiply_divide_2, "10*-10", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0)))), 
        operator: BramaOperatorType::Multiplication, 
        right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(-10.0))))
    }));

    test_compare!(multiply_divide_3, "-10*-10", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(-10.0)))), 
        operator: BramaOperatorType::Multiplication, 
        right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(-10.0))))
    }));

    test_compare!(multiply_divide_4, "-10/-10", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(-10.0)))), 
        operator: BramaOperatorType::Division, 
        right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(-10.0))))
    }));

    test_compare!(multiply_divide_5, "10/10", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0)))), 
        operator: BramaOperatorType::Division, 
        right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0))))
    }));

    test_compare!(multiply_divide_6, "doğru * doğru", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Bool(true)))),
        operator: BramaOperatorType::Multiplication, 
        right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Bool(true)))), 
    }));

    test_compare!(multiply_divide_7, "doğru / doğru", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Bool(true)))),
        operator: BramaOperatorType::Division, 
        right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Bool(true)))), 
    }));

    test_compare!(multiply_divide_8, "1/", Err(BramaError {
        error_type: BramaErrorType::RightSideOfExpressionNotFound,
        column: 2,
        line: 0
    }));
    test_compare!(multiply_divide_9, "/1", Err(BramaError {
        error_type: BramaErrorType::SyntaxError,
        column: 0,
        line: 0
    }));

    test_compare!(modulo_1, "10 mod 10", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0)))), 
        operator: BramaOperatorType::Modulo, 
        right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0))))
    }));

    test_compare!(modulo_2, "10 mod 10", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0)))), 
        operator: BramaOperatorType::Modulo, 
        right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0))))
    }));

    test_compare!(modulo_3, "10 mod 5*2", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0)))),
        operator: BramaOperatorType::Modulo, 
        right: Box::new(BramaAstType::Binary {
            left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(5.0)))),
            operator: BramaOperatorType::Multiplication, 
            right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(2.0))))
        })
    }));

    test_compare!(modulo_4, "5*2 mod 2", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::Binary {
            left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(5.0)))),
            operator: BramaOperatorType::Multiplication, 
            right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(2.0))))
        }),
        operator: BramaOperatorType::Modulo, 
        right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(2.0))))
    }));
}