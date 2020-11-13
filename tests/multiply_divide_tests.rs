extern crate tpd;

#[cfg(test)]
mod tests {
    use crate::tpd::parser::*;
    use crate::tpd::types::*;

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

    test_compare!(multiply_divide_1, "10*10", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::Primative(BramaPrimative::Integer(10))), 
        operator: BramaOperatorType::Multiplication, 
        right: Box::new(BramaAstType::Primative(BramaPrimative::Integer(10)))
    }));

    test_compare!(multiply_divide_2, "10*-10", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::Primative(BramaPrimative::Integer(10))), 
        operator: BramaOperatorType::Multiplication, 
        right: Box::new(BramaAstType::PrefixUnary(BramaOperatorType::Subtraction, Box::new(BramaAstType::Primative(BramaPrimative::Integer(10)))))
    }));

    test_compare!(multiply_divide_3, "-10*-10", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::PrefixUnary(BramaOperatorType::Subtraction, Box::new(BramaAstType::Primative(BramaPrimative::Integer(10))))), 
        operator: BramaOperatorType::Multiplication, 
        right: Box::new(BramaAstType::PrefixUnary(BramaOperatorType::Subtraction, Box::new(BramaAstType::Primative(BramaPrimative::Integer(10)))))
    }));

    test_compare!(multiply_divide_4, "-10/-10", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::PrefixUnary(BramaOperatorType::Subtraction, Box::new(BramaAstType::Primative(BramaPrimative::Integer(10))))), 
        operator: BramaOperatorType::Division, 
        right: Box::new(BramaAstType::PrefixUnary(BramaOperatorType::Subtraction, Box::new(BramaAstType::Primative(BramaPrimative::Integer(10)))))
    }));

    test_compare!(multiply_divide_5, "10/10", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::Primative(BramaPrimative::Integer(10))), 
        operator: BramaOperatorType::Division, 
        right: Box::new(BramaAstType::Primative(BramaPrimative::Integer(10)))
    }));

    test_compare!(multiply_divide_6, "true * true", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::Primative(BramaPrimative::Bool(true))),
        operator: BramaOperatorType::Multiplication, 
        right: Box::new(BramaAstType::Primative(BramaPrimative::Bool(true))), 
    }));

    test_compare!(multiply_divide_7, "true / true", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::Primative(BramaPrimative::Bool(true))),
        operator: BramaOperatorType::Division, 
        right: Box::new(BramaAstType::Primative(BramaPrimative::Bool(true))), 
    }));

    test_compare!(multiply_divide_8, "1/", Err(("Syntax error", 0, 0)));
    test_compare!(multiply_divide_9, "/1", Err(("Syntax error", 0, 0)));
}