extern crate karamellib;

#[cfg(test)]
mod tests {
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

    test_compare!(equality_1, "10 == 10", Ok(Rc::new(KaramelAstType::Control {
        left: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(10.0)))), 
        operator: KaramelOperatorType::Equal, 
        right: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(10.0))))
    })));

    test_compare!(equality_2, "10 != 10", Ok(Rc::new(KaramelAstType::Control {
        left: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(10.0)))), 
        operator: KaramelOperatorType::NotEqual, 
        right: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(10.0))))
    })));

    test_compare!(equality_3, "10+2 != 10", Ok(Rc::new(KaramelAstType::Control {
        left: Rc::new(KaramelAstType::Binary {
            left: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(10.0)))), 
            operator: KaramelOperatorType::Addition, 
            right: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(2.0))))
        }), 
        operator: KaramelOperatorType::NotEqual, 
        right: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(10.0))))
    })));

    test_compare!(equality_4, "10 == 10+2", Ok(Rc::new(KaramelAstType::Control {
        left: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(10.0)))),
        operator: KaramelOperatorType::Equal,
        right: Rc::new(KaramelAstType::Binary {
            left: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(10.0)))), 
            operator: KaramelOperatorType::Addition, 
            right: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(2.0))))
        })
    })));
    
    test_compare!(and_1, "10 ve 10", Ok(Rc::new(KaramelAstType::Control {
        left: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(10.0)))), 
        operator: KaramelOperatorType::And, 
        right: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(10.0))))
    })));
    
    test_compare!(or_1, "10 veya 10", Ok(Rc::new(KaramelAstType::Control {
        left: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(10.0)))), 
        operator: KaramelOperatorType::Or, 
        right: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(10.0))))
    })));
}