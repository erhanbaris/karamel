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

    test_compare!(modulo_1, "10 % 10", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::Primative(BramaPrimative::Integer(10))), 
        operator: BramaOperatorType::Modulo, 
        right: Box::new(BramaAstType::Primative(BramaPrimative::Integer(10)))
    }));

    test_compare!(modulo_2, "10%10", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::Primative(BramaPrimative::Integer(10))), 
        operator: BramaOperatorType::Modulo, 
        right: Box::new(BramaAstType::Primative(BramaPrimative::Integer(10)))
    }));

    test_compare!(modulo_3, "10 % 5*2", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::Primative(BramaPrimative::Integer(10))),
        operator: BramaOperatorType::Modulo, 
        right: Box::new(BramaAstType::Binary {
            left: Box::new(BramaAstType::Primative(BramaPrimative::Integer(5))),
            operator: BramaOperatorType::Multiplication, 
            right: Box::new(BramaAstType::Primative(BramaPrimative::Integer(2)))
        })
    }));

    test_compare!(modulo_4, "5*2%2", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::Binary {
            left: Box::new(BramaAstType::Primative(BramaPrimative::Integer(5))),
            operator: BramaOperatorType::Multiplication, 
            right: Box::new(BramaAstType::Primative(BramaPrimative::Integer(2)))
        }),
        operator: BramaOperatorType::Modulo, 
        right: Box::new(BramaAstType::Primative(BramaPrimative::Integer(2)))
    }));
}