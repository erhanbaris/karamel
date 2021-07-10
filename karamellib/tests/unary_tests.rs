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
    use std::cell::Cell;

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

    test_compare!(unary_1, "+1024", Ok(Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1024.0))))));
    test_compare!(unary_2, "-1024", Ok(Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(-1024.0))))));
    test_compare!(unary_3, "+1024.0", Ok(Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1024.0))))));
    test_compare!(unary_4, "-1024.0", Ok(Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(-1024.0))))));
    test_compare!(unary_5, "değil doğru", Ok(Rc::new(KaramelAstType::PrefixUnary{ 
            operator: KaramelOperatorType::Not, 
            expression: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Bool(true)))), 
            assign_to_temp: Cell::new(false)
        })));
    test_compare!(unary_6, "değil yanlış", Ok(Rc::new(KaramelAstType::PrefixUnary
        { 
            operator: KaramelOperatorType::Not, 
            expression: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Bool(false)))), 
            assign_to_temp: Cell::new(false)
        })));
    test_compare!(unary_7, "değil doğru", Ok(Rc::new(KaramelAstType::PrefixUnary
        { 
            operator: KaramelOperatorType::Not, 
            expression: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Bool(true)))), 
            assign_to_temp: Cell::new(false)
        })));
    test_compare!(unary_8, "değil yanlış", Ok(Rc::new(KaramelAstType::PrefixUnary { 
        operator: KaramelOperatorType::Not, 
        expression: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Bool(false)))), 
        assign_to_temp: Cell::new(false)
    })));
    
    test_compare!(unary_9, "+[]", Err(KaramelError {
        error_type: KaramelErrorType::UnaryWorksWithNumber,
        column: 1,
        line: 0
    }));
    test_compare!(unary_10, "++100", Err(KaramelError {
        error_type: KaramelErrorType::InvalidUnaryOperation,
        column: 2,
        line: 0
    }));
    test_compare!(unary_11, "--100", Err(KaramelError {
        error_type: KaramelErrorType::InvalidUnaryOperation,
        column: 2,
        line: 0
    }));
    test_compare!(unary_12, "--doğru", Err(KaramelError {
        error_type: KaramelErrorType::InvalidUnaryOperation,
        column: 2,
        line: 0
    }));

    test_compare!(unary_13, "++data", Ok(Rc::new(KaramelAstType::PrefixUnary { 
        operator: KaramelOperatorType::Increment, 
        expression: Rc::new(KaramelAstType::Symbol("data".to_string())), 
        assign_to_temp: Cell::new(false)
    })));
    test_compare!(unary_14, "--data", Ok(Rc::new(KaramelAstType::PrefixUnary { 
        operator: KaramelOperatorType::Deccrement, 
        expression: Rc::new(KaramelAstType::Symbol("data".to_string())),
        assign_to_temp: Cell::new(false)
    })));
    test_compare!(unary_15, "--data", Ok(Rc::new(KaramelAstType::PrefixUnary { 
        operator: KaramelOperatorType::Deccrement, 
        expression: Rc::new(KaramelAstType::Symbol("data".to_string())),
        assign_to_temp: Cell::new(false)
    })));
    test_compare!(unary_16, "data--", Ok(Rc::new(KaramelAstType::SuffixUnary(KaramelOperatorType::Deccrement, Rc::new(KaramelAstType::Symbol("data".to_string()))))));
    test_compare!(unary_17, "data++", Ok(Rc::new(KaramelAstType::SuffixUnary(KaramelOperatorType::Increment, Rc::new(KaramelAstType::Symbol("data".to_string()))))));

    test_compare!(unary_18, "+ 1024", Ok(Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1024.0))))));
    test_compare!(unary_19, "++data - 1", Ok(Rc::new(KaramelAstType::Binary {
        left: Rc::new(KaramelAstType::PrefixUnary { 
                operator: KaramelOperatorType::Increment, 
                expression: Rc::new(KaramelAstType::Symbol("data".to_string())), 
                assign_to_temp: Cell::new(false)
            }),
        operator: KaramelOperatorType::Subtraction,
        right: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1.0))))
    })));
    //test_compare!(unary_19, "doğru değil", Ok(Rc::new(KaramelAstType::SuffixUnary(KaramelOperatorType::Not, Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Bool(true)))))));
}