extern crate tpd;

#[cfg(test)]
mod tests {
    use crate::tpd::parser::*;
    use crate::tpd::types::*;
    use crate::tpd::syntax::*;
    use crate::tpd::compiler::value::BramaPrimative;
    use crate::tpd::compiler::ast::BramaAstType;
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

                let syntax = SyntaxParser::new(Box::new(parser.tokens().to_vec()));
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
        right: Box::new(BramaAstType::PrefixUnary(BramaOperatorType::Subtraction, Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0))))))
    }));

    test_compare!(multiply_divide_3, "-10*-10", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::PrefixUnary(BramaOperatorType::Subtraction, Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0)))))), 
        operator: BramaOperatorType::Multiplication, 
        right: Box::new(BramaAstType::PrefixUnary(BramaOperatorType::Subtraction, Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0))))))
    }));

    test_compare!(multiply_divide_4, "-10/-10", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::PrefixUnary(BramaOperatorType::Subtraction, Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0)))))), 
        operator: BramaOperatorType::Division, 
        right: Box::new(BramaAstType::PrefixUnary(BramaOperatorType::Subtraction, Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0))))))
    }));

    test_compare!(multiply_divide_5, "10/10", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0)))), 
        operator: BramaOperatorType::Division, 
        right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0))))
    }));

    test_compare!(multiply_divide_6, "true * true", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Bool(true)))),
        operator: BramaOperatorType::Multiplication, 
        right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Bool(true)))), 
    }));

    test_compare!(multiply_divide_7, "true / true", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Bool(true)))),
        operator: BramaOperatorType::Division, 
        right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Bool(true)))), 
    }));

    test_compare!(multiply_divide_8, "1/", Err(("Symbol parse issue", 0, 0)));
    test_compare!(multiply_divide_9, "/1", Err(("Syntax error, undefined syntax", 0, 1)));



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