extern crate karamellib;

#[cfg(test)]
mod tests {
    use karamellib::error::{KaramelError, KaramelErrorType};

    use crate::karamellib::parser::*;
    use crate::karamellib::types::*;
    use crate::karamellib::syntax::*;
    use crate::karamellib::compiler::value::KaramelPrimative;
    use crate::karamellib::compiler::ast::KaramelAstType;
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

    test_compare!(add_subtract_1, "10 + 10", Ok(Rc::new(KaramelAstType::Binary {
        left: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(10.0)))), 
        operator: KaramelOperatorType::Addition, 
        right: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(10.0))))
    })));

    test_compare!(add_subtract_2, "10 - 10", Ok(Rc::new(KaramelAstType::Binary {
        left: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(10.0)))), 
        operator: KaramelOperatorType::Subtraction, 
        right: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(10.0))))
    })));

    test_compare!(add_subtract_3, "5 * 2 mod 2 - 10", Ok(Rc::new(KaramelAstType::Binary {
        left: Box::new(KaramelAstType::Binary {
            left: Box::new(KaramelAstType::Binary {
                left: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(5.0)))),
                operator: KaramelOperatorType::Multiplication, 
                right: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(2.0))))
            }),
            operator: KaramelOperatorType::Modulo, 
            right: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(2.0))))
        }), 
        operator: KaramelOperatorType::Subtraction, 
        right: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(10.0))))
    })));

    test_compare!(add_subtract_4, "22 + 5 * 2 mod 2", Ok(Rc::new(KaramelAstType::Binary {
        left: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(22.0)))), 
        operator: KaramelOperatorType::Addition, 
        right: Box::new(KaramelAstType::Binary {
            left: Box::new(KaramelAstType::Binary {
                left: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(5.0)))),
                operator: KaramelOperatorType::Multiplication, 
                right: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(2.0))))
            }),
            operator: KaramelOperatorType::Modulo, 
            right: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(2.0))))
        })
    })));

    test_compare!(add_subtract_5, "11 + 12 + 13", Ok(Rc::new(KaramelAstType::Binary {
        left: Box::new(KaramelAstType::Binary {
            left: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(11.0)))), 
            operator: KaramelOperatorType::Addition, 
            right: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(12.0))))
        }), 
        operator: KaramelOperatorType::Addition, 
        right: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(13.0))))
    })));

    test_compare!(multiply_divide_1, "10*10", Ok(Rc::new(KaramelAstType::Binary {
        left: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(10.0)))), 
        operator: KaramelOperatorType::Multiplication, 
        right: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(10.0))))
    })));

    test_compare!(multiply_divide_2, "10*-10", Ok(Rc::new(KaramelAstType::Binary {
        left: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(10.0)))), 
        operator: KaramelOperatorType::Multiplication, 
        right: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(-10.0))))
    })));

    test_compare!(multiply_divide_3, "-10*-10", Ok(Rc::new(KaramelAstType::Binary {
        left: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(-10.0)))), 
        operator: KaramelOperatorType::Multiplication, 
        right: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(-10.0))))
    })));

    test_compare!(multiply_divide_4, "-10/-10", Ok(Rc::new(KaramelAstType::Binary {
        left: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(-10.0)))), 
        operator: KaramelOperatorType::Division, 
        right: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(-10.0))))
    })));

    test_compare!(multiply_divide_5, "10/10", Ok(Rc::new(KaramelAstType::Binary {
        left: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(10.0)))), 
        operator: KaramelOperatorType::Division, 
        right: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(10.0))))
    })));

    test_compare!(multiply_divide_6, "doğru * doğru", Ok(Rc::new(KaramelAstType::Binary {
        left: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Bool(true)))),
        operator: KaramelOperatorType::Multiplication, 
        right: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Bool(true)))), 
    })));

    test_compare!(multiply_divide_7, "doğru / doğru", Ok(Rc::new(KaramelAstType::Binary {
        left: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Bool(true)))),
        operator: KaramelOperatorType::Division, 
        right: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Bool(true)))), 
    })));

    test_compare!(multiply_divide_8, "1/", Err(KaramelError {
        error_type: KaramelErrorType::RightSideOfExpressionNotFound,
        column: 2,
        line: 0
    }));
    test_compare!(multiply_divide_9, "/1", Err(KaramelError {
        error_type: KaramelErrorType::SyntaxError,
        column: 0,
        line: 0
    }));

    test_compare!(modulo_1, "10 mod 10", Ok(Rc::new(KaramelAstType::Binary {
        left: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(10.0)))), 
        operator: KaramelOperatorType::Modulo, 
        right: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(10.0))))
    })));

    test_compare!(modulo_2, "10 mod 10", Ok(Rc::new(KaramelAstType::Binary {
        left: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(10.0)))), 
        operator: KaramelOperatorType::Modulo, 
        right: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(10.0))))
    })));

    test_compare!(modulo_3, "10 mod 5*2", Ok(Rc::new(KaramelAstType::Binary {
        left: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(10.0)))),
        operator: KaramelOperatorType::Modulo, 
        right: Box::new(KaramelAstType::Binary {
            left: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(5.0)))),
            operator: KaramelOperatorType::Multiplication, 
            right: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(2.0))))
        })
    })));

    test_compare!(modulo_4, "5*2 mod 2", Ok(Rc::new(KaramelAstType::Binary {
        left: Box::new(KaramelAstType::Binary {
            left: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(5.0)))),
            operator: KaramelOperatorType::Multiplication, 
            right: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(2.0))))
        }),
        operator: KaramelOperatorType::Modulo, 
        right: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(2.0))))
    })));
}