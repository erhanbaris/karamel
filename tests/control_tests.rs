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

    test_compare!(auqality_1, "10 == 10", Ok(BramaAstType::Control {
        left: Box::new(BramaAstType::Primative(BramaPrimative::Integer(10))), 
        operator: BramaOperatorType::Equal, 
        right: Box::new(BramaAstType::Primative(BramaPrimative::Integer(10)))
    }));

    test_compare!(auqality_2, "10 != 10", Ok(BramaAstType::Control {
        left: Box::new(BramaAstType::Primative(BramaPrimative::Integer(10))), 
        operator: BramaOperatorType::NotEqual, 
        right: Box::new(BramaAstType::Primative(BramaPrimative::Integer(10)))
    }));

    test_compare!(auqality_3, "10+2 != 10", Ok(BramaAstType::Control {
        left: Box::new(BramaAstType::Binary {
            left: Box::new(BramaAstType::Primative(BramaPrimative::Integer(10))), 
            operator: BramaOperatorType::Addition, 
            right: Box::new(BramaAstType::Primative(BramaPrimative::Integer(2)))
        }), 
        operator: BramaOperatorType::NotEqual, 
        right: Box::new(BramaAstType::Primative(BramaPrimative::Integer(10)))
    }));

    test_compare!(auqality_4, "10 == 10+2", Ok(BramaAstType::Control {
        left: Box::new(BramaAstType::Primative(BramaPrimative::Integer(10))),
        operator: BramaOperatorType::Equal,
        right: Box::new(BramaAstType::Binary {
            left: Box::new(BramaAstType::Primative(BramaPrimative::Integer(10))), 
            operator: BramaOperatorType::Addition, 
            right: Box::new(BramaAstType::Primative(BramaPrimative::Integer(2)))
        })
    }));
    
    test_compare!(and_1, "10 and 10", Ok(BramaAstType::Control {
        left: Box::new(BramaAstType::Primative(BramaPrimative::Integer(10))), 
        operator: BramaOperatorType::And, 
        right: Box::new(BramaAstType::Primative(BramaPrimative::Integer(10)))
    }));
    
    test_compare!(or_1, "10 or 10", Ok(BramaAstType::Control {
        left: Box::new(BramaAstType::Primative(BramaPrimative::Integer(10))), 
        operator: BramaOperatorType::Or, 
        right: Box::new(BramaAstType::Primative(BramaPrimative::Integer(10)))
    }));
}