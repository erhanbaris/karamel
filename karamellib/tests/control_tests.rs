extern crate karamellib;

#[cfg(test)]
mod tests {
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

    test_compare!(equality_1, "10 == 10", Ok(BramaAstType::Control {
        left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0)))), 
        operator: BramaOperatorType::Equal, 
        right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0))))
    }));

    test_compare!(equality_2, "10 != 10", Ok(BramaAstType::Control {
        left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0)))), 
        operator: BramaOperatorType::NotEqual, 
        right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0))))
    }));

    test_compare!(equality_3, "10+2 eşitdeğildir 10", Ok(BramaAstType::Control {
        left: Box::new(BramaAstType::Binary {
            left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0)))), 
            operator: BramaOperatorType::Addition, 
            right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(2.0))))
        }), 
        operator: BramaOperatorType::NotEqual, 
        right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0))))
    }));

    test_compare!(equality_4, "10 eşittir 10+2", Ok(BramaAstType::Control {
        left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0)))),
        operator: BramaOperatorType::Equal,
        right: Box::new(BramaAstType::Binary {
            left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0)))), 
            operator: BramaOperatorType::Addition, 
            right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(2.0))))
        })
    }));
    
    test_compare!(and_1, "10 ve 10", Ok(BramaAstType::Control {
        left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0)))), 
        operator: BramaOperatorType::And, 
        right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0))))
    }));
    
    test_compare!(or_1, "10 veya 10", Ok(BramaAstType::Control {
        left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0)))), 
        operator: BramaOperatorType::Or, 
        right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0))))
    }));
}