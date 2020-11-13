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

    test_compare!(add_subtract_1, "10 + 10", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::Primative(BramaPrimative::Integer(10))), 
        operator: BramaOperatorType::Addition, 
        right: Box::new(BramaAstType::Primative(BramaPrimative::Integer(10)))
    }));

    test_compare!(add_subtract_2, "10 - 10", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::Primative(BramaPrimative::Integer(10))), 
        operator: BramaOperatorType::Subtraction, 
        right: Box::new(BramaAstType::Primative(BramaPrimative::Integer(10)))
    }));

    test_compare!(add_subtract_3, "5 * 2 % 2 - 10", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::Binary {
            left: Box::new(BramaAstType::Binary {
                left: Box::new(BramaAstType::Primative(BramaPrimative::Integer(5))),
                operator: BramaOperatorType::Multiplication, 
                right: Box::new(BramaAstType::Primative(BramaPrimative::Integer(2)))
            }),
            operator: BramaOperatorType::Modulo, 
            right: Box::new(BramaAstType::Primative(BramaPrimative::Integer(2)))
        }), 
        operator: BramaOperatorType::Subtraction, 
        right: Box::new(BramaAstType::Primative(BramaPrimative::Integer(10)))
    }));

    test_compare!(add_subtract_4, "22 + 5 * 2 % 2", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::Primative(BramaPrimative::Integer(22))), 
        operator: BramaOperatorType::Addition, 
        right: Box::new(BramaAstType::Binary {
            left: Box::new(BramaAstType::Binary {
                left: Box::new(BramaAstType::Primative(BramaPrimative::Integer(5))),
                operator: BramaOperatorType::Multiplication, 
                right: Box::new(BramaAstType::Primative(BramaPrimative::Integer(2)))
            }),
            operator: BramaOperatorType::Modulo, 
            right: Box::new(BramaAstType::Primative(BramaPrimative::Integer(2)))
        })
    }));
}